//! Proc macros for type-safe Buildkite pipeline DSL
//!
//! This crate provides the `pipeline!` macro for declaratively defining
//! Buildkite pipelines with compile-time validation.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use std::collections::HashSet;
use syn::{
    braced, bracketed,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Error, Ident, LitStr, Result, Token,
};

/// Strip the `r#` prefix from raw identifiers.
/// This allows users to write `r#if` or `r#async` to use Rust keywords as field names.
fn strip_raw_ident(s: &str) -> &str {
    s.strip_prefix("r#").unwrap_or(s)
}

/// Represents a nested value that can be a literal, object, or array.
/// Used for parsing complex fields like retry, plugins, build, etc.
#[derive(Clone)]
enum NestedValue {
    String(String),
    Int(i64),
    Bool(bool),
    Object(Vec<(String, NestedValue)>),
    Array(Vec<NestedValue>),
}

impl NestedValue {
    /// Parse a nested value from a ParseStream
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(syn::token::Brace) {
            let content;
            braced!(content in input);
            let mut pairs = Vec::new();
            while !content.is_empty() {
                let key = if content.peek(LitStr) {
                    let lit: LitStr = content.parse()?;
                    lit.value()
                } else {
                    let ident: Ident = content.parse()?;
                    ident.to_string()
                };
                content.parse::<Token![:]>()?;
                let value = NestedValue::parse(&content)?;
                pairs.push((key, value));
                if content.peek(Token![,]) {
                    content.parse::<Token![,]>()?;
                }
            }
            Ok(NestedValue::Object(pairs))
        } else if input.peek(syn::token::Bracket) {
            let content;
            bracketed!(content in input);
            let mut items = Vec::new();
            while !content.is_empty() {
                items.push(NestedValue::parse(&content)?);
                if content.peek(Token![,]) {
                    content.parse::<Token![,]>()?;
                }
            }
            Ok(NestedValue::Array(items))
        } else if input.peek(LitStr) {
            let lit: LitStr = input.parse()?;
            Ok(NestedValue::String(lit.value()))
        } else if input.peek(syn::LitInt) {
            let lit: syn::LitInt = input.parse()?;
            Ok(NestedValue::Int(lit.base10_parse()?))
        } else if input.peek(syn::LitBool) {
            let lit: syn::LitBool = input.parse()?;
            Ok(NestedValue::Bool(lit.value()))
        } else if input.peek(Ident) {
            let ident: Ident = input.parse()?;
            match ident.to_string().as_str() {
                "true" => Ok(NestedValue::Bool(true)),
                "false" => Ok(NestedValue::Bool(false)),
                other => Ok(NestedValue::String(other.to_string())),
            }
        } else {
            Err(Error::new(input.span(), "expected value (string, number, bool, object, or array)"))
        }
    }

    /// Convert to a serde_json::Value TokenStream
    fn to_json_tokens(&self) -> TokenStream2 {
        match self {
            NestedValue::String(s) => {
                quote! { ::serde_json::Value::String(#s.to_string()) }
            }
            NestedValue::Int(i) => {
                quote! { ::serde_json::Value::Number(::serde_json::Number::from(#i)) }
            }
            NestedValue::Bool(b) => {
                quote! { ::serde_json::Value::Bool(#b) }
            }
            NestedValue::Object(pairs) => {
                let inserts: Vec<TokenStream2> = pairs
                    .iter()
                    .map(|(k, v)| {
                        let value_tokens = v.to_json_tokens();
                        quote! { __obj.insert(#k.to_string(), #value_tokens); }
                    })
                    .collect();
                quote! {
                    {
                        let mut __obj = ::serde_json::Map::new();
                        #(#inserts)*
                        ::serde_json::Value::Object(__obj)
                    }
                }
            }
            NestedValue::Array(items) => {
                let item_tokens: Vec<TokenStream2> = items.iter().map(|v| v.to_json_tokens()).collect();
                quote! {
                    ::serde_json::Value::Array(vec![#(#item_tokens),*])
                }
            }
        }
    }
}

/// A declarative macro for building type-safe Buildkite pipelines.
///
/// # Example
///
/// ```ignore
/// use rust_buildkite::pipeline;
///
/// let p = pipeline! {
///     env: {
///         CI: "true",
///         NODE_ENV: "test"
///     },
///     steps: [
///         command("echo hello").label("Say Hello").key("hello"),
///         command("npm test").label("Tests").key("tests").depends_on("hello"),
///         wait,
///         block("Deploy to Production?"),
///         command("./deploy.sh").depends_on("tests")
///     ]
/// };
/// ```
#[proc_macro]
pub fn pipeline(input: TokenStream) -> TokenStream {
    let pipeline_def = parse_macro_input!(input as PipelineDef);

    match pipeline_def.generate() {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Top-level pipeline definition
struct PipelineDef {
    env: Option<Vec<(Ident, LitStr)>>,
    steps: Vec<StepDef>,
}

impl Parse for PipelineDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut env = None;
        let mut steps = Vec::new();

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![:]>()?;

            match key.to_string().as_str() {
                "env" => {
                    let content;
                    braced!(content in input);
                    let mut env_vars = Vec::new();
                    while !content.is_empty() {
                        let var_name: Ident = content.parse()?;
                        content.parse::<Token![:]>()?;
                        let var_value: LitStr = content.parse()?;
                        env_vars.push((var_name, var_value));
                        if content.peek(Token![,]) {
                            content.parse::<Token![,]>()?;
                        }
                    }
                    env = Some(env_vars);
                }
                "steps" => {
                    let content;
                    bracketed!(content in input);
                    let step_list: Punctuated<StepDef, Token![,]> =
                        Punctuated::parse_terminated(&content)?;
                    steps = step_list.into_iter().collect();
                }
                other => {
                    return Err(Error::new(
                        key.span(),
                        format!("unknown pipeline field: {}", other),
                    ));
                }
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(PipelineDef { env, steps })
    }
}

impl PipelineDef {
    fn generate(&self) -> Result<TokenStream2> {
        let mut keys: HashSet<String> = HashSet::new();
        let mut key_spans: Vec<(String, proc_macro2::Span)> = Vec::new();

        for step in &self.steps {
            if let Some((key, span)) = step.get_key() {
                if keys.contains(&key) {
                    return Err(Error::new(span, format!("duplicate step key: '{}'", key)));
                }
                keys.insert(key.clone());
                key_spans.push((key, span));
            }
        }
        for step in &self.steps {
            for (dep, span) in step.get_depends_on() {
                if !keys.contains(&dep) {
                    let available: Vec<_> = keys.iter().collect();
                    return Err(Error::new(
                        span,
                        format!(
                            "unknown step key '{}' in depends_on. Available keys: {:?}",
                            dep, available
                        ),
                    ));
                }
            }
        }
        let step_tokens: Vec<TokenStream2> = self
            .steps
            .iter()
            .map(|s| s.to_token_stream())
            .collect();
        let env_tokens = if let Some(env_vars) = &self.env {
            let env_inserts: Vec<TokenStream2> = env_vars
                .iter()
                .map(|(k, v)| {
                    let key_str = k.to_string();
                    quote! {
                        __env_map.insert(
                            #key_str.to_string(),
                            ::serde_json::Value::String(#v.to_string())
                        );
                    }
                })
                .collect();

            quote! {
                {
                    let mut __env_map = ::serde_json::Map::new();
                    #(#env_inserts)*
                    Some(::rust_buildkite::Env(__env_map))
                }
            }
        } else {
            quote! { None }
        };

        Ok(quote! {
            {
                let __result: ::rust_buildkite::JsonSchemaForBuildkitePipelineConfigurationFiles = 
                    ::rust_buildkite::JsonSchemaForBuildkitePipelineConfigurationFiles::builder()
                        .steps(::rust_buildkite::PipelineSteps(vec![
                            #(#step_tokens),*
                        ]))
                        .env(#env_tokens)
                        .try_into()
                        .expect("pipeline construction failed");
                __result
            }
        })
    }
}

/// Individual step definition
enum StepDef {
    Command(CommandStepDef),
    Wait(WaitStepDef),
    Block(BlockStepDef),
    Input(InputStepDef),
    Trigger(TriggerStepDef),
    Group(GroupStepDef),
}

impl Parse for StepDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident: Ident = input.parse()?;
        let ident_str = ident.to_string();

        match ident_str.as_str() {
            "command" => {
                if input.peek(syn::token::Brace) {
                    Self::parse_command_object_literal(input)
                } else if input.peek(syn::token::Paren) {
                    Self::parse_command_fluent(input)
                } else {
                    Err(Error::new(
                        ident.span(),
                        "expected '(' or '{' after 'command'",
                    ))
                }
            }
            "wait" => {
                if input.peek(syn::token::Brace) {
                    Self::parse_wait_object_literal(input)
                } else {
                    Ok(StepDef::Wait(WaitStepDef::default()))
                }
            }
            "block" => {
                if input.peek(syn::token::Brace) {
                    Self::parse_block_object_literal(input)
                } else if input.peek(syn::token::Paren) {
                    Self::parse_block_fluent(input)
                } else {
                    Err(Error::new(
                        ident.span(),
                        "expected '(' or '{' after 'block'",
                    ))
                }
            }
            "input" => {
                if input.peek(syn::token::Brace) {
                    Self::parse_input_object_literal(input)
                } else if input.peek(syn::token::Paren) {
                    Self::parse_input_fluent(input)
                } else {
                    Err(Error::new(
                        ident.span(),
                        "expected '(' or '{' after 'input'",
                    ))
                }
            }
            "trigger" => {
                if input.peek(syn::token::Brace) {
                    Self::parse_trigger_object_literal(input)
                } else if input.peek(syn::token::Paren) {
                    Self::parse_trigger_fluent(input)
                } else {
                    Err(Error::new(
                        ident.span(),
                        "expected '(' or '{' after 'trigger'",
                    ))
                }
            }
            "group" => {
                if input.peek(syn::token::Brace) {
                    Self::parse_group_object_literal(input)
                } else if input.peek(syn::token::Paren) {
                    Self::parse_group_fluent(input)
                } else {
                    Err(Error::new(
                        ident.span(),
                        "expected '(' or '{' after 'group'",
                    ))
                }
            }
            other => Err(Error::new(
                ident.span(),
                format!(
                    "unknown step type: '{}'. Expected: command, wait, block, input, trigger, group",
                    other
                ),
            )),
        }
    }
}

impl StepDef {
    fn get_key(&self) -> Option<(String, proc_macro2::Span)> {
        match self {
            StepDef::Command(c) => c.key.clone(),
            StepDef::Block(b) => b.key.clone(),
            StepDef::Input(i) => i.key.clone(),
            StepDef::Trigger(t) => t.key.clone(),
            StepDef::Group(g) => g.key.clone(),
            StepDef::Wait(_) => None,
        }
    }

    fn get_depends_on(&self) -> Vec<(String, proc_macro2::Span)> {
        match self {
            StepDef::Command(c) => c.depends_on.clone(),
            StepDef::Block(b) => b.depends_on.clone(),
            StepDef::Input(i) => i.depends_on.clone(),
            StepDef::Trigger(t) => t.depends_on.clone(),
            StepDef::Group(g) => g.depends_on.clone(),
            StepDef::Wait(w) => w.depends_on.clone(),
        }
    }

    /// Parse command step with fluent syntax: command("...").method()
    fn parse_command_fluent(input: ParseStream) -> Result<Self> {
        let content;
        syn::parenthesized!(content in input);
        let cmd: LitStr = content.parse()?;
        let mut step = CommandStepDef::new(cmd);
        while input.peek(Token![.]) {
            input.parse::<Token![.]>()?;
            let method: Ident = input.parse()?;
            let args;
            syn::parenthesized!(args in input);

            match strip_raw_ident(&method.to_string()) {
                "label" => {
                    step.label = Some(args.parse()?);
                }
                "key" => {
                    let key_lit: LitStr = args.parse()?;
                    step.key = Some((key_lit.value(), key_lit.span()));
                }
                "depends_on" => {
                    let dep: LitStr = args.parse()?;
                    step.depends_on.push((dep.value(), dep.span()));
                }
                "env" => {
                    let var_name: Ident = args.parse()?;
                    args.parse::<Token![,]>()?;
                    let var_value: LitStr = args.parse()?;
                    step.env.push((var_name.to_string(), var_value));
                }
                "timeout_in_minutes" => {
                    let timeout: syn::LitInt = args.parse()?;
                    step.timeout_in_minutes = Some(timeout);
                }
                "soft_fail" => {
                    step.soft_fail = true;
                }
                "parallelism" => {
                    let p: syn::LitInt = args.parse()?;
                    step.parallelism = Some(p);
                }
                "artifact_paths" => {
                    let path: LitStr = args.parse()?;
                    step.artifact_paths.push(path);
                }
                "agents" => {
                    if args.peek(syn::token::Brace) {
                        let agents_content;
                        braced!(agents_content in args);
                        while !agents_content.is_empty() {
                            let agent_key: Ident = agents_content.parse()?;
                            agents_content.parse::<Token![:]>()?;
                            let agent_value: LitStr = agents_content.parse()?;
                            step.agents.push((agent_key.to_string(), agent_value));
                            if agents_content.peek(Token![,]) {
                                agents_content.parse::<Token![,]>()?;
                            }
                        }
                    } else {
                        let agent_key: Ident = args.parse()?;
                        args.parse::<Token![,]>()?;
                        let agent_value: LitStr = args.parse()?;
                        step.agents.push((agent_key.to_string(), agent_value));
                    }
                }
                "branches" => {
                    let branch: LitStr = args.parse()?;
                    step.branches.push(branch);
                }
                "if" => {
                    let condition: LitStr = args.parse()?;
                    step.if_condition = Some(condition);
                }
                "cache" => {
                    let path: LitStr = args.parse()?;
                    step.cache.push(path);
                }
                "retry" => {
                    let retry_value = NestedValue::parse(&args)?;
                    if let NestedValue::Object(pairs) = retry_value {
                        let mut config = RetryConfig::default();
                        for (k, v) in pairs {
                            match k.as_str() {
                                "automatic" => config.automatic = Some(v),
                                "manual" => config.manual = Some(v),
                                _ => {}
                            }
                        }
                        step.retry = Some(config);
                    }
                }
                "retry_automatic" => {
                    let limit: syn::LitInt = args.parse()?;
                    let limit_val: i64 = limit.base10_parse()?;
                    let config = RetryConfig {
                        automatic: Some(NestedValue::Object(vec![
                            ("limit".to_string(), NestedValue::Int(limit_val)),
                        ])),
                        manual: None,
                    };
                    step.retry = Some(config);
                }
                "plugin" => {
                    let name: LitStr = args.parse()?;
                    args.parse::<Token![,]>()?;
                    let config = NestedValue::parse(&args)?;
                    step.plugins.push(NestedValue::Object(vec![
                        (name.value(), config),
                    ]));
                }
                "notify_slack" => {
                    let channel: LitStr = args.parse()?;
                    step.notify.push(NestedValue::Object(vec![
                        ("slack".to_string(), NestedValue::String(channel.value())),
                    ]));
                }
                "matrix" => {
                    let matrix_value = NestedValue::parse(&args)?;
                    step.matrix = Some(matrix_value);
                }
                "concurrency" => {
                    let c: syn::LitInt = args.parse()?;
                    step.concurrency = Some(c);
                }
                "concurrency_group" => {
                    let group: LitStr = args.parse()?;
                    step.concurrency_group = Some(group);
                }
                "skip" => {
                    if args.is_empty() {
                        step.skip = Some(SkipValue::Bool(true));
                    } else {
                        let reason: LitStr = args.parse()?;
                        step.skip = Some(SkipValue::Reason(reason.value()));
                    }
                }
                "priority" => {
                    let p: syn::LitInt = args.parse()?;
                    step.priority = Some(p);
                }
                "allow_dependency_failure" => {
                    step.allow_dependency_failure = true;
                }
                other => {
                    return Err(Error::new(
                        method.span(),
                        format!("unknown command step method: {}", other),
                    ));
                }
            }
        }

        Ok(StepDef::Command(step))
    }

    /// Parse command step with object-literal syntax: command { command: "...", label: "..." }
    fn parse_command_object_literal(input: ParseStream) -> Result<Self> {
        let content;
        braced!(content in input);

        let mut step = CommandStepDef::new_empty();

        while !content.is_empty() {
            let field: Ident = content.parse()?;
            content.parse::<Token![:]>()?;

            match strip_raw_ident(&field.to_string()) {
                "command" => {
                    let cmd: LitStr = content.parse()?;
                    step.command = Some(cmd);
                }
                "label" => {
                    step.label = Some(content.parse()?);
                }
                "key" => {
                    let key_lit: LitStr = content.parse()?;
                    step.key = Some((key_lit.value(), key_lit.span()));
                }
                "env" => {
                    let env_content;
                    braced!(env_content in content);
                    while !env_content.is_empty() {
                        let var_name: Ident = env_content.parse()?;
                        env_content.parse::<Token![:]>()?;
                        let var_value: LitStr = env_content.parse()?;
                        step.env.push((var_name.to_string(), var_value));
                        if env_content.peek(Token![,]) {
                            env_content.parse::<Token![,]>()?;
                        }
                    }
                }
                "depends_on" => {
                    let deps_content;
                    bracketed!(deps_content in content);
                    while !deps_content.is_empty() {
                        let dep: LitStr = deps_content.parse()?;
                        step.depends_on.push((dep.value(), dep.span()));
                        if deps_content.peek(Token![,]) {
                            deps_content.parse::<Token![,]>()?;
                        }
                    }
                }
                "timeout_in_minutes" => {
                    let timeout: syn::LitInt = content.parse()?;
                    step.timeout_in_minutes = Some(timeout);
                }
                "soft_fail" => {
                    let val: syn::LitBool = content.parse()?;
                    step.soft_fail = val.value();
                }
                "parallelism" => {
                    let p: syn::LitInt = content.parse()?;
                    step.parallelism = Some(p);
                }
                "artifact_paths" => {
                    let paths_content;
                    bracketed!(paths_content in content);
                    while !paths_content.is_empty() {
                        let path: LitStr = paths_content.parse()?;
                        step.artifact_paths.push(path);
                        if paths_content.peek(Token![,]) {
                            paths_content.parse::<Token![,]>()?;
                        }
                    }
                }
                "agents" => {
                    let agents_content;
                    braced!(agents_content in content);
                    while !agents_content.is_empty() {
                        let agent_key: Ident = agents_content.parse()?;
                        agents_content.parse::<Token![:]>()?;
                        let agent_value: LitStr = agents_content.parse()?;
                        step.agents.push((agent_key.to_string(), agent_value));
                        if agents_content.peek(Token![,]) {
                            agents_content.parse::<Token![,]>()?;
                        }
                    }
                }
                "branches" => {
                    let branches_content;
                    bracketed!(branches_content in content);
                    while !branches_content.is_empty() {
                        let branch: LitStr = branches_content.parse()?;
                        step.branches.push(branch);
                        if branches_content.peek(Token![,]) {
                            branches_content.parse::<Token![,]>()?;
                        }
                    }
                }
                "condition" | "if" => {
                    let condition: LitStr = content.parse()?;
                    step.if_condition = Some(condition);
                }
                "cache" => {
                    let cache_content;
                    bracketed!(cache_content in content);
                    while !cache_content.is_empty() {
                        let path: LitStr = cache_content.parse()?;
                        step.cache.push(path);
                        if cache_content.peek(Token![,]) {
                            cache_content.parse::<Token![,]>()?;
                        }
                    }
                }
                "retry" => {
                    let retry_value = NestedValue::parse(&content)?;
                    if let NestedValue::Object(pairs) = retry_value {
                        let mut config = RetryConfig::default();
                        for (k, v) in pairs {
                            match k.as_str() {
                                "automatic" => config.automatic = Some(v),
                                "manual" => config.manual = Some(v),
                                _ => {}
                            }
                        }
                        step.retry = Some(config);
                    }
                }
                "plugins" => {
                    let plugins_content;
                    bracketed!(plugins_content in content);
                    while !plugins_content.is_empty() {
                        let plugin = NestedValue::parse(&plugins_content)?;
                        step.plugins.push(plugin);
                        if plugins_content.peek(Token![,]) {
                            plugins_content.parse::<Token![,]>()?;
                        }
                    }
                }
                "notify" => {
                    let notify_content;
                    bracketed!(notify_content in content);
                    while !notify_content.is_empty() {
                        let notification = NestedValue::parse(&notify_content)?;
                        step.notify.push(notification);
                        if notify_content.peek(Token![,]) {
                            notify_content.parse::<Token![,]>()?;
                        }
                    }
                }
                "matrix" => {
                    let matrix_value = NestedValue::parse(&content)?;
                    step.matrix = Some(matrix_value);
                }
                "concurrency" => {
                    let c: syn::LitInt = content.parse()?;
                    step.concurrency = Some(c);
                }
                "concurrency_group" => {
                    let group: LitStr = content.parse()?;
                    step.concurrency_group = Some(group);
                }
                "skip" => {
                    if content.peek(syn::LitBool) {
                        let val: syn::LitBool = content.parse()?;
                        step.skip = Some(SkipValue::Bool(val.value()));
                    } else {
                        let reason: LitStr = content.parse()?;
                        step.skip = Some(SkipValue::Reason(reason.value()));
                    }
                }
                "priority" => {
                    let p: syn::LitInt = content.parse()?;
                    step.priority = Some(p);
                }
                "allow_dependency_failure" => {
                    let val: syn::LitBool = content.parse()?;
                    step.allow_dependency_failure = val.value();
                }
                other => {
                    return Err(Error::new(
                        field.span(),
                        format!("unknown command step field: {}", other),
                    ));
                }
            }
            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            }
        }
        if step.command.is_none() {
            return Err(Error::new(
                input.span(),
                "command step requires 'command' field",
            ));
        }

        Ok(StepDef::Command(step))
    }

    /// Parse wait step with object-literal syntax: wait { continue_on_failure: true }
    fn parse_wait_object_literal(input: ParseStream) -> Result<Self> {
        let content;
        braced!(content in input);

        let mut step = WaitStepDef::default();

        while !content.is_empty() {
            let field: Ident = content.parse()?;
            content.parse::<Token![:]>()?;

            match strip_raw_ident(&field.to_string()) {
                "continue_on_failure" => {
                    let val: syn::LitBool = content.parse()?;
                    step.continue_on_failure = val.value();
                }
                "depends_on" => {
                    let dep: LitStr = content.parse()?;
                    step.depends_on.push((dep.value(), dep.span()));
                }
                "if" => {
                    let condition: LitStr = content.parse()?;
                    step.if_condition = Some(condition.value());
                }
                other => {
                    return Err(Error::new(
                        field.span(),
                        format!("unknown wait step field: {}", other),
                    ));
                }
            }

            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            }
        }

        Ok(StepDef::Wait(step))
    }

    /// Parse block step with fluent syntax: block("...").method()
    fn parse_block_fluent(input: ParseStream) -> Result<Self> {
        let content;
        syn::parenthesized!(content in input);
        let prompt: LitStr = content.parse()?;
        let mut step = BlockStepDef::new(prompt);

        while input.peek(Token![.]) {
            input.parse::<Token![.]>()?;
            let method: Ident = input.parse()?;
            let args;
            syn::parenthesized!(args in input);

            match strip_raw_ident(&method.to_string()) {
                "key" => {
                    let key_lit: LitStr = args.parse()?;
                    step.key = Some((key_lit.value(), key_lit.span()));
                }
                "depends_on" => {
                    let dep: LitStr = args.parse()?;
                    step.depends_on.push((dep.value(), dep.span()));
                }
                "allowed_teams" => {
                    let team: LitStr = args.parse()?;
                    step.allowed_teams.push(team.value());
                }
                "blocked_state" => {
                    let state: LitStr = args.parse()?;
                    step.blocked_state = Some(state.value());
                }
                "branches" => {
                    let branch: LitStr = args.parse()?;
                    step.branches.push(branch);
                }
                "if" => {
                    let condition: LitStr = args.parse()?;
                    step.if_condition = Some(condition);
                }
                "prompt" => {
                    let p: LitStr = args.parse()?;
                    step.prompt_text = Some(p);
                }
                "allow_dependency_failure" => {
                    step.allow_dependency_failure = true;
                }
                "field" => {
                    let field = FieldDef::parse(&args)?;
                    step.fields.push(field);
                }
                other => {
                    return Err(Error::new(
                        method.span(),
                        format!("unknown block step method: {}", other),
                    ));
                }
            }
        }

        Ok(StepDef::Block(step))
    }

    /// Parse block step with object-literal syntax: block { block: "...", key: "..." }
    fn parse_block_object_literal(input: ParseStream) -> Result<Self> {
        let content;
        braced!(content in input);

        let mut step = BlockStepDef::new_empty();

        while !content.is_empty() {
            let field: Ident = content.parse()?;
            content.parse::<Token![:]>()?;

            match strip_raw_ident(&field.to_string()) {
                "block" => {
                    let prompt: LitStr = content.parse()?;
                    step.prompt = Some(prompt);
                }
                "key" => {
                    let key_lit: LitStr = content.parse()?;
                    step.key = Some((key_lit.value(), key_lit.span()));
                }
                "depends_on" => {
                    let deps_content;
                    bracketed!(deps_content in content);
                    while !deps_content.is_empty() {
                        let dep: LitStr = deps_content.parse()?;
                        step.depends_on.push((dep.value(), dep.span()));
                        if deps_content.peek(Token![,]) {
                            deps_content.parse::<Token![,]>()?;
                        }
                    }
                }
                "fields" => {
                    let fields_content;
                    bracketed!(fields_content in content);
                    while !fields_content.is_empty() {
                        let field_def = FieldDef::parse(&fields_content)?;
                        step.fields.push(field_def);
                        if fields_content.peek(Token![,]) {
                            fields_content.parse::<Token![,]>()?;
                        }
                    }
                }
                "allowed_teams" => {
                    if content.peek(syn::token::Bracket) {
                        let teams_content;
                        bracketed!(teams_content in content);
                        while !teams_content.is_empty() {
                            let team: LitStr = teams_content.parse()?;
                            step.allowed_teams.push(team.value());
                            if teams_content.peek(Token![,]) {
                                teams_content.parse::<Token![,]>()?;
                            }
                        }
                    } else {
                        let team: LitStr = content.parse()?;
                        step.allowed_teams.push(team.value());
                    }
                }
                "blocked_state" => {
                    let state: LitStr = content.parse()?;
                    step.blocked_state = Some(state.value());
                }
                "branches" => {
                    let branches_content;
                    bracketed!(branches_content in content);
                    while !branches_content.is_empty() {
                        let branch: LitStr = branches_content.parse()?;
                        step.branches.push(branch);
                        if branches_content.peek(Token![,]) {
                            branches_content.parse::<Token![,]>()?;
                        }
                    }
                }
                "if" => {
                    let condition: LitStr = content.parse()?;
                    step.if_condition = Some(condition);
                }
                "prompt" => {
                    let p: LitStr = content.parse()?;
                    step.prompt_text = Some(p);
                }
                "allow_dependency_failure" => {
                    let val: syn::LitBool = content.parse()?;
                    step.allow_dependency_failure = val.value();
                }
                other => {
                    return Err(Error::new(
                        field.span(),
                        format!("unknown block step field: {}", other),
                    ));
                }
            }

            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            }
        }

        if step.prompt.is_none() {
            return Err(Error::new(
                input.span(),
                "block step requires 'block' field",
            ));
        }

        Ok(StepDef::Block(step))
    }

    /// Parse input step with fluent syntax: input("...").method()
    fn parse_input_fluent(input: ParseStream) -> Result<Self> {
        let content;
        syn::parenthesized!(content in input);
        let prompt: LitStr = content.parse()?;
        let mut step = InputStepDef::new(prompt);

        while input.peek(Token![.]) {
            input.parse::<Token![.]>()?;
            let method: Ident = input.parse()?;
            let args;
            syn::parenthesized!(args in input);

            match strip_raw_ident(&method.to_string()) {
                "key" => {
                    let key_lit: LitStr = args.parse()?;
                    step.key = Some((key_lit.value(), key_lit.span()));
                }
                "depends_on" => {
                    let dep: LitStr = args.parse()?;
                    step.depends_on.push((dep.value(), dep.span()));
                }
                "allowed_teams" => {
                    let team: LitStr = args.parse()?;
                    step.allowed_teams.push(team.value());
                }
                "blocked_state" => {
                    let state: LitStr = args.parse()?;
                    step.blocked_state = Some(state.value());
                }
                "branches" => {
                    let branch: LitStr = args.parse()?;
                    step.branches.push(branch);
                }
                "if" => {
                    let condition: LitStr = args.parse()?;
                    step.if_condition = Some(condition);
                }
                "prompt" => {
                    let p: LitStr = args.parse()?;
                    step.prompt_text = Some(p);
                }
                "allow_dependency_failure" => {
                    step.allow_dependency_failure = true;
                }
                "field" => {
                    let field = FieldDef::parse(&args)?;
                    step.fields.push(field);
                }
                other => {
                    return Err(Error::new(
                        method.span(),
                        format!("unknown input step method: {}", other),
                    ));
                }
            }
        }

        Ok(StepDef::Input(step))
    }

    /// Parse input step with object-literal syntax: input { input: "...", key: "..." }
    fn parse_input_object_literal(input: ParseStream) -> Result<Self> {
        let content;
        braced!(content in input);

        let mut step = InputStepDef::new_empty();

        while !content.is_empty() {
            let field: Ident = content.parse()?;
            content.parse::<Token![:]>()?;

            match strip_raw_ident(&field.to_string()) {
                "input" => {
                    let prompt: LitStr = content.parse()?;
                    step.prompt = Some(prompt);
                }
                "key" => {
                    let key_lit: LitStr = content.parse()?;
                    step.key = Some((key_lit.value(), key_lit.span()));
                }
                "depends_on" => {
                    let deps_content;
                    bracketed!(deps_content in content);
                    while !deps_content.is_empty() {
                        let dep: LitStr = deps_content.parse()?;
                        step.depends_on.push((dep.value(), dep.span()));
                        if deps_content.peek(Token![,]) {
                            deps_content.parse::<Token![,]>()?;
                        }
                    }
                }
                "fields" => {
                    let fields_content;
                    bracketed!(fields_content in content);
                    while !fields_content.is_empty() {
                        let field_def = FieldDef::parse(&fields_content)?;
                        step.fields.push(field_def);
                        if fields_content.peek(Token![,]) {
                            fields_content.parse::<Token![,]>()?;
                        }
                    }
                }
                "allowed_teams" => {
                    if content.peek(syn::token::Bracket) {
                        let teams_content;
                        bracketed!(teams_content in content);
                        while !teams_content.is_empty() {
                            let team: LitStr = teams_content.parse()?;
                            step.allowed_teams.push(team.value());
                            if teams_content.peek(Token![,]) {
                                teams_content.parse::<Token![,]>()?;
                            }
                        }
                    } else {
                        let team: LitStr = content.parse()?;
                        step.allowed_teams.push(team.value());
                    }
                }
                "blocked_state" => {
                    let state: LitStr = content.parse()?;
                    step.blocked_state = Some(state.value());
                }
                "branches" => {
                    let branches_content;
                    bracketed!(branches_content in content);
                    while !branches_content.is_empty() {
                        let branch: LitStr = branches_content.parse()?;
                        step.branches.push(branch);
                        if branches_content.peek(Token![,]) {
                            branches_content.parse::<Token![,]>()?;
                        }
                    }
                }
                "if" => {
                    let condition: LitStr = content.parse()?;
                    step.if_condition = Some(condition);
                }
                "prompt" => {
                    let p: LitStr = content.parse()?;
                    step.prompt_text = Some(p);
                }
                "allow_dependency_failure" => {
                    let val: syn::LitBool = content.parse()?;
                    step.allow_dependency_failure = val.value();
                }
                other => {
                    return Err(Error::new(
                        field.span(),
                        format!("unknown input step field: {}", other),
                    ));
                }
            }

            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            }
        }

        if step.prompt.is_none() {
            return Err(Error::new(
                input.span(),
                "input step requires 'input' field",
            ));
        }

        Ok(StepDef::Input(step))
    }

    /// Parse trigger step with fluent syntax: trigger("...").method()
    fn parse_trigger_fluent(input: ParseStream) -> Result<Self> {
        let content;
        syn::parenthesized!(content in input);
        let pipeline: LitStr = content.parse()?;
        let mut step = TriggerStepDef::new(pipeline);

        while input.peek(Token![.]) {
            input.parse::<Token![.]>()?;
            let method: Ident = input.parse()?;
            let args;
            syn::parenthesized!(args in input);

            match strip_raw_ident(&method.to_string()) {
                "key" => {
                    let key_lit: LitStr = args.parse()?;
                    step.key = Some((key_lit.value(), key_lit.span()));
                }
                "depends_on" => {
                    let dep: LitStr = args.parse()?;
                    step.depends_on.push((dep.value(), dep.span()));
                }
                "label" => {
                    step.label = Some(args.parse()?);
                }
                "async" => {
                    step.async_trigger = true;
                }
                "build" => {
                    let build_value = NestedValue::parse(&args)?;
                    if let NestedValue::Object(pairs) = build_value {
                        let mut config = TriggerBuildConfig::default();
                        for (k, v) in pairs {
                            match k.as_str() {
                                "branch" => {
                                    if let NestedValue::String(s) = v {
                                        config.branch = Some(s);
                                    }
                                }
                                "commit" => {
                                    if let NestedValue::String(s) = v {
                                        config.commit = Some(s);
                                    }
                                }
                                "message" => {
                                    if let NestedValue::String(s) = v {
                                        config.message = Some(s);
                                    }
                                }
                                "env" => {
                                    if let NestedValue::Object(env_pairs) = v {
                                        for (ek, ev) in env_pairs {
                                            if let NestedValue::String(es) = ev {
                                                config.env.push((ek, es));
                                            }
                                        }
                                    }
                                }
                                "meta_data" => {
                                    if let NestedValue::Object(md_pairs) = v {
                                        for (mk, mv) in md_pairs {
                                            if let NestedValue::String(ms) = mv {
                                                config.meta_data.push((mk, ms));
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        step.build = Some(config);
                    }
                }
                "branches" => {
                    let branch: LitStr = args.parse()?;
                    step.branches.push(branch);
                }
                "if" => {
                    let condition: LitStr = args.parse()?;
                    step.if_condition = Some(condition);
                }
                "skip" => {
                    if args.is_empty() {
                        step.skip = Some(SkipValue::Bool(true));
                    } else {
                        let reason: LitStr = args.parse()?;
                        step.skip = Some(SkipValue::Reason(reason.value()));
                    }
                }
                "soft_fail" => {
                    step.soft_fail = true;
                }
                "allow_dependency_failure" => {
                    step.allow_dependency_failure = true;
                }
                other => {
                    return Err(Error::new(
                        method.span(),
                        format!("unknown trigger step method: {}", other),
                    ));
                }
            }
        }

        Ok(StepDef::Trigger(step))
    }

    /// Parse trigger step with object-literal syntax: trigger { trigger: "...", async: true }
    fn parse_trigger_object_literal(input: ParseStream) -> Result<Self> {
        let content;
        braced!(content in input);

        let mut step = TriggerStepDef::new_empty();

        while !content.is_empty() {
            let field: Ident = content.parse()?;
            content.parse::<Token![:]>()?;

            match strip_raw_ident(&field.to_string()) {
                "trigger" => {
                    let pipeline: LitStr = content.parse()?;
                    step.pipeline = Some(pipeline);
                }
                "label" => {
                    step.label = Some(content.parse()?);
                }
                "key" => {
                    let key_lit: LitStr = content.parse()?;
                    step.key = Some((key_lit.value(), key_lit.span()));
                }
                "async" => {
                    let val: syn::LitBool = content.parse()?;
                    step.async_trigger = val.value();
                }
                "depends_on" => {
                    let deps_content;
                    bracketed!(deps_content in content);
                    while !deps_content.is_empty() {
                        let dep: LitStr = deps_content.parse()?;
                        step.depends_on.push((dep.value(), dep.span()));
                        if deps_content.peek(Token![,]) {
                            deps_content.parse::<Token![,]>()?;
                        }
                    }
                }
                "build" => {
                    let build_value = NestedValue::parse(&content)?;
                    if let NestedValue::Object(pairs) = build_value {
                        let mut config = TriggerBuildConfig::default();
                        for (k, v) in pairs {
                            match k.as_str() {
                                "branch" => {
                                    if let NestedValue::String(s) = v {
                                        config.branch = Some(s);
                                    }
                                }
                                "commit" => {
                                    if let NestedValue::String(s) = v {
                                        config.commit = Some(s);
                                    }
                                }
                                "message" => {
                                    if let NestedValue::String(s) = v {
                                        config.message = Some(s);
                                    }
                                }
                                "env" => {
                                    if let NestedValue::Object(env_pairs) = v {
                                        for (ek, ev) in env_pairs {
                                            if let NestedValue::String(es) = ev {
                                                config.env.push((ek, es));
                                            }
                                        }
                                    }
                                }
                                "meta_data" => {
                                    if let NestedValue::Object(md_pairs) = v {
                                        for (mk, mv) in md_pairs {
                                            if let NestedValue::String(ms) = mv {
                                                config.meta_data.push((mk, ms));
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        step.build = Some(config);
                    }
                }
                "branches" => {
                    let branches_content;
                    bracketed!(branches_content in content);
                    while !branches_content.is_empty() {
                        let branch: LitStr = branches_content.parse()?;
                        step.branches.push(branch);
                        if branches_content.peek(Token![,]) {
                            branches_content.parse::<Token![,]>()?;
                        }
                    }
                }
                "if" => {
                    let condition: LitStr = content.parse()?;
                    step.if_condition = Some(condition);
                }
                "skip" => {
                    if content.peek(syn::LitBool) {
                        let val: syn::LitBool = content.parse()?;
                        step.skip = Some(SkipValue::Bool(val.value()));
                    } else {
                        let reason: LitStr = content.parse()?;
                        step.skip = Some(SkipValue::Reason(reason.value()));
                    }
                }
                "soft_fail" => {
                    let val: syn::LitBool = content.parse()?;
                    step.soft_fail = val.value();
                }
                "allow_dependency_failure" => {
                    let val: syn::LitBool = content.parse()?;
                    step.allow_dependency_failure = val.value();
                }
                other => {
                    return Err(Error::new(
                        field.span(),
                        format!("unknown trigger step field: {}", other),
                    ));
                }
            }

            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            }
        }

        if step.pipeline.is_none() {
            return Err(Error::new(
                input.span(),
                "trigger step requires 'trigger' field",
            ));
        }

        Ok(StepDef::Trigger(step))
    }

    /// Parse group step with fluent syntax: group("...").steps([...])
    fn parse_group_fluent(input: ParseStream) -> Result<Self> {
        let content;
        syn::parenthesized!(content in input);
        let label: LitStr = content.parse()?;
        let mut step = GroupStepDef::new(label);

        while input.peek(Token![.]) {
            input.parse::<Token![.]>()?;
            let method: Ident = input.parse()?;
            let args;
            syn::parenthesized!(args in input);

            match strip_raw_ident(&method.to_string()) {
                "key" => {
                    let key_lit: LitStr = args.parse()?;
                    step.key = Some((key_lit.value(), key_lit.span()));
                }
                "depends_on" => {
                    let dep: LitStr = args.parse()?;
                    step.depends_on.push((dep.value(), dep.span()));
                }
                "steps" => {
                    let nested;
                    bracketed!(nested in args);
                    let nested_steps: Punctuated<StepDef, Token![,]> =
                        Punctuated::parse_terminated(&nested)?;
                    step.steps = nested_steps.into_iter().collect();
                }
                "if" => {
                    let condition: LitStr = args.parse()?;
                    step.if_condition = Some(condition);
                }
                "skip" => {
                    if args.is_empty() {
                        step.skip = Some(SkipValue::Bool(true));
                    } else {
                        let reason: LitStr = args.parse()?;
                        step.skip = Some(SkipValue::Reason(reason.value()));
                    }
                }
                "notify_slack" => {
                    let channel: LitStr = args.parse()?;
                    step.notify.push(NestedValue::Object(vec![
                        ("slack".to_string(), NestedValue::String(channel.value())),
                    ]));
                }
                "notify" => {
                    let notify_value = NestedValue::parse(&args)?;
                    if let NestedValue::Array(items) = notify_value {
                        step.notify.extend(items);
                    } else {
                        step.notify.push(notify_value);
                    }
                }
                "allow_dependency_failure" => {
                    step.allow_dependency_failure = true;
                }
                other => {
                    return Err(Error::new(
                        method.span(),
                        format!("unknown group step method: {}", other),
                    ));
                }
            }
        }

        Ok(StepDef::Group(step))
    }

    /// Parse group step with object-literal syntax: group { group: "...", steps: [...] }
    fn parse_group_object_literal(input: ParseStream) -> Result<Self> {
        let content;
        braced!(content in input);

        let mut step = GroupStepDef::new_empty();

        while !content.is_empty() {
            let field: Ident = content.parse()?;
            content.parse::<Token![:]>()?;

            match strip_raw_ident(&field.to_string()) {
                "group" => {
                    let label: LitStr = content.parse()?;
                    step.label = Some(label);
                }
                "key" => {
                    let key_lit: LitStr = content.parse()?;
                    step.key = Some((key_lit.value(), key_lit.span()));
                }
                "depends_on" => {
                    let deps_content;
                    bracketed!(deps_content in content);
                    while !deps_content.is_empty() {
                        let dep: LitStr = deps_content.parse()?;
                        step.depends_on.push((dep.value(), dep.span()));
                        if deps_content.peek(Token![,]) {
                            deps_content.parse::<Token![,]>()?;
                        }
                    }
                }
                "steps" => {
                    let nested;
                    bracketed!(nested in content);
                    let nested_steps: Punctuated<StepDef, Token![,]> =
                        Punctuated::parse_terminated(&nested)?;
                    step.steps = nested_steps.into_iter().collect();
                }
                "if" => {
                    let condition: LitStr = content.parse()?;
                    step.if_condition = Some(condition);
                }
                "skip" => {
                    if content.peek(syn::LitBool) {
                        let val: syn::LitBool = content.parse()?;
                        step.skip = Some(SkipValue::Bool(val.value()));
                    } else {
                        let reason: LitStr = content.parse()?;
                        step.skip = Some(SkipValue::Reason(reason.value()));
                    }
                }
                "notify" => {
                    let notify_content;
                    bracketed!(notify_content in content);
                    while !notify_content.is_empty() {
                        let notification = NestedValue::parse(&notify_content)?;
                        step.notify.push(notification);
                        if notify_content.peek(Token![,]) {
                            notify_content.parse::<Token![,]>()?;
                        }
                    }
                }
                "allow_dependency_failure" => {
                    let val: syn::LitBool = content.parse()?;
                    step.allow_dependency_failure = val.value();
                }
                other => {
                    return Err(Error::new(
                        field.span(),
                        format!("unknown group step field: {}", other),
                    ));
                }
            }

            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            }
        }

        if step.label.is_none() {
            return Err(Error::new(
                input.span(),
                "group step requires 'group' field",
            ));
        }

        Ok(StepDef::Group(step))
    }
}

impl ToTokens for StepDef {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let step_tokens = match self {
            StepDef::Command(c) => c.to_tokens_inner(),
            StepDef::Wait(w) => w.to_tokens_inner(),
            StepDef::Block(b) => b.to_tokens_inner(),
            StepDef::Input(i) => i.to_tokens_inner(),
            StepDef::Trigger(t) => t.to_tokens_inner(),
            StepDef::Group(g) => g.to_tokens_inner(),
        };
        tokens.extend(step_tokens);
    }
}

impl StepDef {
    /// Generate tokens for this step as a GroupStepsItem (used for nested steps in groups)
    fn to_group_step_tokens(&self) -> TokenStream2 {
        match self {
            StepDef::Command(c) => c.to_group_step_tokens(),
            StepDef::Wait(w) => w.to_group_step_tokens(),
            StepDef::Block(b) => b.to_group_step_tokens(),
            StepDef::Input(i) => i.to_group_step_tokens(),
            StepDef::Trigger(t) => t.to_group_step_tokens(),
            StepDef::Group(_) => {
                quote! { compile_error!("Groups cannot be nested inside other groups") }
            }
        }
    }
}
#[derive(Default)]
struct WaitStepDef {
    continue_on_failure: bool,
    depends_on: Vec<(String, proc_macro2::Span)>,
    if_condition: Option<String>,
}

impl WaitStepDef {
    fn to_tokens_inner(&self) -> TokenStream2 {
        if self.continue_on_failure || self.if_condition.is_some() {
            let continue_on_failure_tokens = if self.continue_on_failure {
                quote! { .continue_on_failure(::rust_buildkite::WaitStepContinueOnFailure::Boolean(true)) }
            } else {
                quote! {}
            };
            
            let if_tokens = if let Some(condition) = &self.if_condition {
                quote! { .if_(::rust_buildkite::If(#condition.to_string())) }
            } else {
                quote! {}
            };
            
            quote! {
                ::rust_buildkite::PipelineStepsItem::WaitStep(
                    ::rust_buildkite::WaitStep::builder()
                        #continue_on_failure_tokens
                        #if_tokens
                        .try_into()
                        .expect("wait step construction failed")
                )
            }
        } else {
            quote! {
                ::rust_buildkite::PipelineStepsItem::StringWaitStep(
                    ::rust_buildkite::StringWaitStep::Wait
                )
            }
        }
    }

    fn to_group_step_tokens(&self) -> TokenStream2 {
        if self.continue_on_failure || self.if_condition.is_some() {
            let continue_on_failure_tokens = if self.continue_on_failure {
                quote! { .continue_on_failure(::rust_buildkite::WaitStepContinueOnFailure::Boolean(true)) }
            } else {
                quote! {}
            };
            
            let if_tokens = if let Some(condition) = &self.if_condition {
                quote! { .if_(::rust_buildkite::If(#condition.to_string())) }
            } else {
                quote! {}
            };
            
            quote! {
                ::rust_buildkite::GroupStepsItem::WaitStep(
                    ::rust_buildkite::WaitStep::builder()
                        #continue_on_failure_tokens
                        #if_tokens
                        .try_into()
                        .expect("wait step construction failed")
                )
            }
        } else {
            quote! {
                ::rust_buildkite::GroupStepsItem::StringWaitStep(
                    ::rust_buildkite::StringWaitStep::Wait
                )
            }
        }
    }
}
struct CommandStepDef {
    command: Option<LitStr>,
    label: Option<LitStr>,
    key: Option<(String, proc_macro2::Span)>,
    depends_on: Vec<(String, proc_macro2::Span)>,
    env: Vec<(String, LitStr)>,
    timeout_in_minutes: Option<syn::LitInt>,
    soft_fail: bool,
    parallelism: Option<syn::LitInt>,
    artifact_paths: Vec<LitStr>,
    agents: Vec<(String, LitStr)>,
    branches: Vec<LitStr>,
    if_condition: Option<LitStr>,
    cache: Vec<LitStr>,
    retry: Option<RetryConfig>,
    plugins: Vec<NestedValue>,
    notify: Vec<NestedValue>,
    matrix: Option<NestedValue>,
    concurrency: Option<syn::LitInt>,
    concurrency_group: Option<LitStr>,
    skip: Option<SkipValue>,
    priority: Option<syn::LitInt>,
    allow_dependency_failure: bool,
}

/// Retry configuration for command steps
#[derive(Clone, Default)]
struct RetryConfig {
    automatic: Option<NestedValue>,
    manual: Option<NestedValue>,
}

/// Skip value - can be bool or string reason
#[derive(Clone)]
enum SkipValue {
    Bool(bool),
    Reason(String),
}

/// Field definition for block/input steps
#[derive(Clone)]
enum FieldDef {
    Text(TextFieldDef),
    Select(SelectFieldDef),
}

/// Text field for block/input steps
#[derive(Clone, Default)]
struct TextFieldDef {
    key: String,
    text: Option<String>,
    hint: Option<String>,
    required: Option<bool>,
    default: Option<String>,
    format: Option<String>,
}

/// Select field for block/input steps
#[derive(Clone, Default)]
struct SelectFieldDef {
    key: String,
    select: Option<String>,
    hint: Option<String>,
    required: Option<bool>,
    default: Option<String>,
    multiple: Option<bool>,
    options: Vec<SelectFieldOptionDef>,
}

/// Option for select field
#[derive(Clone)]
struct SelectFieldOptionDef {
    label: String,
    value: String,
}

impl FieldDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let field_type: Ident = input.parse()?;
        let content;
        braced!(content in input);
        
        match field_type.to_string().as_str() {
            "text" => {
                let mut field = TextFieldDef::default();
                while !content.is_empty() {
                    let key: Ident = content.parse()?;
                    content.parse::<Token![:]>()?;
                    match key.to_string().as_str() {
                        "key" => {
                            let k: LitStr = content.parse()?;
                            field.key = k.value();
                        }
                        "text" | "label" => {
                            let t: LitStr = content.parse()?;
                            field.text = Some(t.value());
                        }
                        "hint" => {
                            let h: LitStr = content.parse()?;
                            field.hint = Some(h.value());
                        }
                        "required" => {
                            let r: syn::LitBool = content.parse()?;
                            field.required = Some(r.value());
                        }
                        "default" => {
                            let d: LitStr = content.parse()?;
                            field.default = Some(d.value());
                        }
                        "format" => {
                            let f: LitStr = content.parse()?;
                            field.format = Some(f.value());
                        }
                        other => {
                            return Err(Error::new(key.span(), format!("unknown text field property: {}", other)));
                        }
                    }
                    if content.peek(Token![,]) {
                        content.parse::<Token![,]>()?;
                    }
                }
                Ok(FieldDef::Text(field))
            }
            "select" => {
                let mut field = SelectFieldDef::default();
                while !content.is_empty() {
                    let key: Ident = content.parse()?;
                    content.parse::<Token![:]>()?;
                    match key.to_string().as_str() {
                        "key" => {
                            let k: LitStr = content.parse()?;
                            field.key = k.value();
                        }
                        "select" | "label" => {
                            let s: LitStr = content.parse()?;
                            field.select = Some(s.value());
                        }
                        "hint" => {
                            let h: LitStr = content.parse()?;
                            field.hint = Some(h.value());
                        }
                        "required" => {
                            let r: syn::LitBool = content.parse()?;
                            field.required = Some(r.value());
                        }
                        "default" => {
                            let d: LitStr = content.parse()?;
                            field.default = Some(d.value());
                        }
                        "multiple" => {
                            let m: syn::LitBool = content.parse()?;
                            field.multiple = Some(m.value());
                        }
                        "options" => {
                            let options_content;
                            bracketed!(options_content in content);
                            while !options_content.is_empty() {
                                let opt_content;
                                braced!(opt_content in options_content);
                                let mut label = String::new();
                                let mut value = String::new();
                                while !opt_content.is_empty() {
                                    let opt_key: Ident = opt_content.parse()?;
                                    opt_content.parse::<Token![:]>()?;
                                    match opt_key.to_string().as_str() {
                                        "label" => {
                                            let l: LitStr = opt_content.parse()?;
                                            label = l.value();
                                        }
                                        "value" => {
                                            let v: LitStr = opt_content.parse()?;
                                            value = v.value();
                                        }
                                        _ => {}
                                    }
                                    if opt_content.peek(Token![,]) {
                                        opt_content.parse::<Token![,]>()?;
                                    }
                                }
                                field.options.push(SelectFieldOptionDef { label, value });
                                if options_content.peek(Token![,]) {
                                    options_content.parse::<Token![,]>()?;
                                }
                            }
                        }
                        other => {
                            return Err(Error::new(key.span(), format!("unknown select field property: {}", other)));
                        }
                    }
                    if content.peek(Token![,]) {
                        content.parse::<Token![,]>()?;
                    }
                }
                Ok(FieldDef::Select(field))
            }
            other => Err(Error::new(field_type.span(), format!("unknown field type: {}. Expected 'text' or 'select'", other))),
        }
    }

    fn to_tokens_inner(&self) -> TokenStream2 {
        match self {
            FieldDef::Text(f) => {
                let key = &f.key;
                let text_tokens = if let Some(t) = &f.text {
                    quote! { .text(Some(#t.to_string())) }
                } else {
                    quote! {}
                };
                let hint_tokens = if let Some(h) = &f.hint {
                    quote! { .hint(Some(#h.to_string())) }
                } else {
                    quote! {}
                };
                let required_tokens = if let Some(r) = f.required {
                    quote! { .required(::rust_buildkite::TextFieldRequired::Boolean(#r)) }
                } else {
                    quote! {}
                };
                let default_tokens = if let Some(d) = &f.default {
                    quote! { .default(Some(#d.to_string())) }
                } else {
                    quote! {}
                };
                let format_tokens = if let Some(fmt) = &f.format {
                    quote! { .format(Some(#fmt.to_string())) }
                } else {
                    quote! {}
                };
                quote! {
                    ::rust_buildkite::FieldsItem::TextField(
                        ::rust_buildkite::TextField::builder()
                            .key(#key.parse::<::rust_buildkite::TextFieldKey>().expect("invalid key"))
                            #text_tokens
                            #hint_tokens
                            #required_tokens
                            #default_tokens
                            #format_tokens
                            .try_into()
                            .expect("text field construction failed")
                    )
                }
            }
            FieldDef::Select(f) => {
                let key = &f.key;
                let select_tokens = if let Some(s) = &f.select {
                    quote! { .select(Some(#s.to_string())) }
                } else {
                    quote! {}
                };
                let hint_tokens = if let Some(h) = &f.hint {
                    quote! { .hint(Some(#h.to_string())) }
                } else {
                    quote! {}
                };
                let required_tokens = if let Some(r) = f.required {
                    quote! { .required(::rust_buildkite::SelectFieldRequired::Boolean(#r)) }
                } else {
                    quote! {}
                };
                let default_tokens = if let Some(d) = &f.default {
                    quote! { .default(::rust_buildkite::SelectFieldDefault::String(#d.to_string())) }
                } else {
                    quote! {}
                };
                let multiple_tokens = if let Some(m) = f.multiple {
                    quote! { .multiple(::rust_buildkite::SelectFieldMultiple::Boolean(#m)) }
                } else {
                    quote! {}
                };
                let options: Vec<TokenStream2> = f.options.iter().map(|opt| {
                    let label = &opt.label;
                    let value = &opt.value;
                    quote! {
                        ::rust_buildkite::SelectFieldOption::builder()
                            .label(#label.to_string())
                            .value(#value.to_string())
                            .try_into()
                            .expect("select option construction failed")
                    }
                }).collect();
                quote! {
                    ::rust_buildkite::FieldsItem::SelectField(
                        ::rust_buildkite::SelectField::builder()
                            .key(#key.parse::<::rust_buildkite::SelectFieldKey>().expect("invalid key"))
                            #select_tokens
                            #hint_tokens
                            #required_tokens
                            #default_tokens
                            #multiple_tokens
                            .options(vec![#(#options),*])
                            .try_into()
                            .expect("select field construction failed")
                    )
                }
            }
        }
    }
}

impl CommandStepDef {
    fn new(command: LitStr) -> Self {
        Self {
            command: Some(command),
            label: None,
            key: None,
            depends_on: Vec::new(),
            env: Vec::new(),
            timeout_in_minutes: None,
            soft_fail: false,
            parallelism: None,
            artifact_paths: Vec::new(),
            agents: Vec::new(),
            branches: Vec::new(),
            if_condition: None,
            cache: Vec::new(),
            retry: None,
            plugins: Vec::new(),
            notify: Vec::new(),
            matrix: None,
            concurrency: None,
            concurrency_group: None,
            skip: None,
            priority: None,
            allow_dependency_failure: false,
        }
    }

    fn new_empty() -> Self {
        Self {
            command: None,
            label: None,
            key: None,
            depends_on: Vec::new(),
            env: Vec::new(),
            timeout_in_minutes: None,
            soft_fail: false,
            parallelism: None,
            artifact_paths: Vec::new(),
            agents: Vec::new(),
            branches: Vec::new(),
            if_condition: None,
            cache: Vec::new(),
            retry: None,
            plugins: Vec::new(),
            notify: Vec::new(),
            matrix: None,
            concurrency: None,
            concurrency_group: None,
            skip: None,
            priority: None,
            allow_dependency_failure: false,
        }
    }

    fn to_tokens_inner(&self) -> TokenStream2 {
        let cmd = self.command.as_ref().expect("command must be set");

        let label_tokens = if let Some(label) = &self.label {
            quote! { .label(Some(::rust_buildkite::Label(#label.to_string()))) }
        } else {
            quote! {}
        };

        let key_tokens = if let Some((key, _)) = &self.key {
            quote! { .key(Some(#key.to_string().try_into().expect("invalid key"))) }
        } else {
            quote! {}
        };

        let depends_on_tokens = if !self.depends_on.is_empty() {
            let deps: Vec<_> = self.depends_on.iter().map(|(d, _)| d).collect();
            quote! {
                .depends_on(Some(::rust_buildkite::DependsOn::DependsOnList(
                    ::rust_buildkite::DependsOnList(vec![
                        #(::rust_buildkite::DependsOnListItem::String(#deps.to_string())),*
                    ])
                )))
            }
        } else {
            quote! {}
        };

        let timeout_tokens = if let Some(timeout) = &self.timeout_in_minutes {
            quote! { .timeout_in_minutes(Some(::std::num::NonZeroU64::new(#timeout).expect("timeout must be > 0"))) }
        } else {
            quote! {}
        };

        let soft_fail_tokens = if self.soft_fail {
            quote! { .soft_fail(Some(::rust_buildkite::SoftFail::Variant0(::rust_buildkite::SoftFailVariant0::Boolean(true)))) }
        } else {
            quote! {}
        };

        let parallelism_tokens = if let Some(p) = &self.parallelism {
            quote! { .parallelism(Some(#p)) }
        } else {
            quote! {}
        };

        let artifact_tokens = if !self.artifact_paths.is_empty() {
            let paths = &self.artifact_paths;
            quote! {
                .artifact_paths(Some(::rust_buildkite::CommandStepArtifactPaths::Array(vec![
                    #(#paths.to_string()),*
                ])))
            }
        } else {
            quote! {}
        };

        let env_tokens = if !self.env.is_empty() {
            let env_inserts: Vec<TokenStream2> = self
                .env
                .iter()
                .map(|(k, v)| {
                    quote! {
                        __step_env.insert(#k.to_string(), ::serde_json::Value::String(#v.to_string()));
                    }
                })
                .collect();

            quote! {
                .env({
                    let mut __step_env = ::serde_json::Map::new();
                    #(#env_inserts)*
                    Some(::rust_buildkite::Env(__step_env))
                })
            }
        } else {
            quote! {}
        };

        let agents_tokens = if !self.agents.is_empty() {
            let agent_inserts: Vec<TokenStream2> = self
                .agents
                .iter()
                .map(|(k, v)| {
                    quote! {
                        __step_agents.insert(#k.to_string(), ::serde_json::Value::String(#v.to_string()));
                    }
                })
                .collect();

            quote! {
                .agents({
                    let mut __step_agents = ::serde_json::Map::new();
                    #(#agent_inserts)*
                    Some(::rust_buildkite::Agents::Object(::rust_buildkite::AgentsObject(__step_agents)))
                })
            }
        } else {
            quote! {}
        };

        let branches_tokens = if !self.branches.is_empty() {
            let branches = &self.branches;
            quote! {
                .branches(Some(::rust_buildkite::Branches::Array(vec![
                    #(#branches.to_string()),*
                ])))
            }
        } else {
            quote! {}
        };

        let if_tokens = if let Some(condition) = &self.if_condition {
            quote! { .if_(Some(::rust_buildkite::If(#condition.to_string()))) }
        } else {
            quote! {}
        };

        let cache_tokens = if !self.cache.is_empty() {
            let paths = &self.cache;
            quote! {
                .cache(Some(::rust_buildkite::Cache::Array(vec![
                    #(#paths.to_string()),*
                ])))
            }
        } else {
            quote! {}
        };

        let retry_tokens = if let Some(retry) = &self.retry {
            let automatic_tokens = if let Some(auto) = &retry.automatic {
                let auto_json = auto.to_json_tokens();
                quote! {
                    __retry_obj.insert("automatic".to_string(), #auto_json);
                }
            } else {
                quote! {}
            };
            let manual_tokens = if let Some(manual) = &retry.manual {
                let manual_json = manual.to_json_tokens();
                quote! {
                    __retry_obj.insert("manual".to_string(), #manual_json);
                }
            } else {
                quote! {}
            };
            quote! {
                .retry({
                    let mut __retry_obj = ::serde_json::Map::new();
                    #automatic_tokens
                    #manual_tokens
                    let __retry_value = ::serde_json::Value::Object(__retry_obj);
                    Some(::serde_json::from_value(__retry_value).expect("invalid retry config"))
                })
            }
        } else {
            quote! {}
        };

        let plugins_tokens = if !self.plugins.is_empty() {
            let plugin_values: Vec<TokenStream2> = self.plugins.iter().map(|p| p.to_json_tokens()).collect();
            quote! {
                .plugins({
                    let __plugins_array = vec![#(#plugin_values),*];
                    Some(::rust_buildkite::Plugins::List(
                        ::rust_buildkite::PluginsList(__plugins_array.into_iter().map(|v| {
                            ::serde_json::from_value(v).expect("invalid plugin")
                        }).collect())
                    ))
                })
            }
        } else {
            quote! {}
        };

        let notify_tokens = if !self.notify.is_empty() {
            let notify_values: Vec<TokenStream2> = self.notify.iter().map(|n| n.to_json_tokens()).collect();
            quote! {
                .notify({
                    let __notify_array = vec![#(#notify_values),*];
                    Some(::rust_buildkite::CommandStepNotify(__notify_array.into_iter().map(|v| {
                        ::serde_json::from_value(v).expect("invalid notify")
                    }).collect()))
                })
            }
        } else {
            quote! {}
        };

        let matrix_tokens = if let Some(matrix) = &self.matrix {
            let matrix_json = matrix.to_json_tokens();
            quote! {
                .matrix({
                    let __matrix_value = #matrix_json;
                    Some(::serde_json::from_value(__matrix_value).expect("invalid matrix"))
                })
            }
        } else {
            quote! {}
        };

        let concurrency_tokens = if let Some(c) = &self.concurrency {
            quote! { .concurrency(Some(#c)) }
        } else {
            quote! {}
        };

        let concurrency_group_tokens = if let Some(group) = &self.concurrency_group {
            quote! { .concurrency_group(Some(#group.to_string())) }
        } else {
            quote! {}
        };

        let skip_tokens = match &self.skip {
            Some(SkipValue::Bool(true)) => {
                quote! { .skip(Some(::rust_buildkite::Skip::Boolean(true))) }
            }
            Some(SkipValue::Bool(false)) => {
                quote! { .skip(Some(::rust_buildkite::Skip::Boolean(false))) }
            }
            Some(SkipValue::Reason(reason)) => {
                quote! { .skip(Some(::rust_buildkite::Skip::String(#reason.parse().expect("invalid skip reason")))) }
            }
            None => quote! {},
        };

        let priority_tokens = if let Some(p) = &self.priority {
            quote! { .priority(Some(::rust_buildkite::Priority(#p))) }
        } else {
            quote! {}
        };

        let allow_dependency_failure_tokens = if self.allow_dependency_failure {
            quote! { .allow_dependency_failure(Some(::rust_buildkite::AllowDependencyFailure::Boolean(true))) }
        } else {
            quote! {}
        };

        quote! {
            ::rust_buildkite::PipelineStepsItem::CommandStep(
                ::rust_buildkite::CommandStep::builder()
                    .command(Some(::rust_buildkite::CommandStepCommand::String(#cmd.to_string())))
                    #label_tokens
                    #key_tokens
                    #depends_on_tokens
                    #timeout_tokens
                    #soft_fail_tokens
                    #parallelism_tokens
                    #artifact_tokens
                    #env_tokens
                    #agents_tokens
                    #branches_tokens
                    #if_tokens
                    #cache_tokens
                    #retry_tokens
                    #plugins_tokens
                    #notify_tokens
                    #matrix_tokens
                    #concurrency_tokens
                    #concurrency_group_tokens
                    #skip_tokens
                    #priority_tokens
                    #allow_dependency_failure_tokens
                    .try_into()
                    .expect("command step construction failed")
            )
        }
    }

    fn to_group_step_tokens(&self) -> TokenStream2 {
        let cmd = self.command.as_ref().expect("command must be set");

        let label_tokens = if let Some(l) = &self.label {
            quote! { .label(Some(::rust_buildkite::Label(#l.to_string()))) }
        } else {
            quote! {}
        };

        let key_tokens = if let Some((key, _)) = &self.key {
            quote! { .key(Some(#key.to_string().try_into().expect("invalid key"))) }
        } else {
            quote! {}
        };

        let depends_on_tokens = if !self.depends_on.is_empty() {
            let deps: Vec<_> = self.depends_on.iter().map(|(d, _)| d).collect();
            quote! {
                .depends_on(Some(::rust_buildkite::DependsOn::DependsOnList(
                    ::rust_buildkite::DependsOnList(vec![
                        #(::rust_buildkite::DependsOnListItem::String(#deps.to_string())),*
                    ])
                )))
            }
        } else {
            quote! {}
        };

        let timeout_tokens = if let Some(t) = &self.timeout_in_minutes {
            quote! { .timeout_in_minutes(Some(#t)) }
        } else {
            quote! {}
        };

        let soft_fail_tokens = if self.soft_fail {
            quote! { .soft_fail(Some(::rust_buildkite::SoftFail::Variant0(::rust_buildkite::SoftFailVariant0::Boolean(true)))) }
        } else {
            quote! {}
        };

        let parallelism_tokens = if let Some(p) = &self.parallelism {
            quote! { .parallelism(Some(#p)) }
        } else {
            quote! {}
        };

        let artifact_tokens = if !self.artifact_paths.is_empty() {
            let paths: Vec<_> = self.artifact_paths.iter().collect();
            quote! {
                .artifact_paths(Some(::rust_buildkite::CommandStepArtifactPaths::Array(vec![
                    #(#paths.to_string()),*
                ])))
            }
        } else {
            quote! {}
        };

        let env_tokens = if !self.env.is_empty() {
            let inserts: Vec<TokenStream2> = self.env.iter().map(|(k, v)| {
                quote! { __env_map.insert(#k.to_string(), ::serde_json::Value::String(#v.to_string())); }
            }).collect();
            quote! {
                .env({
                    let mut __env_map = ::serde_json::Map::new();
                    #(#inserts)*
                    Some(::rust_buildkite::Env(__env_map))
                })
            }
        } else {
            quote! {}
        };

        let agents_tokens = if !self.agents.is_empty() {
            let inserts: Vec<TokenStream2> = self.agents.iter().map(|(k, v)| {
                quote! { __agents_map.insert(#k.to_string(), ::serde_json::Value::String(#v.to_string())); }
            }).collect();
            quote! {
                .agents({
                    let mut __agents_map = ::serde_json::Map::new();
                    #(#inserts)*
                    Some(::rust_buildkite::Agents::Object(::rust_buildkite::AgentsObject(__agents_map)))
                })
            }
        } else {
            quote! {}
        };

        let branches_tokens = if !self.branches.is_empty() {
            let branches: Vec<_> = self.branches.iter().collect();
            quote! {
                .branches(Some(::rust_buildkite::Branches::Array(vec![
                    #(#branches.to_string()),*
                ])))
            }
        } else {
            quote! {}
        };

        let if_tokens = if let Some(condition) = &self.if_condition {
            quote! { .if_(Some(::rust_buildkite::If(#condition.to_string()))) }
        } else {
            quote! {}
        };

        let cache_tokens = if !self.cache.is_empty() {
            let paths: Vec<_> = self.cache.iter().collect();
            quote! {
                .cache(Some(::rust_buildkite::Cache::Array(vec![
                    #(#paths.to_string()),*
                ])))
            }
        } else {
            quote! {}
        };

        let retry_tokens = if let Some(retry) = &self.retry {
            let automatic_tokens = if let Some(auto) = &retry.automatic {
                let auto_json = auto.to_json_tokens();
                quote! {
                    .automatic({
                        let __auto_value = #auto_json;
                        Some(::serde_json::from_value(__auto_value).expect("invalid automatic retry config"))
                    })
                }
            } else {
                quote! {}
            };
            let manual_tokens = if let Some(manual) = &retry.manual {
                let manual_json = manual.to_json_tokens();
                quote! {
                    .manual({
                        let __manual_value = #manual_json;
                        Some(::serde_json::from_value(__manual_value).expect("invalid manual retry config"))
                    })
                }
            } else {
                quote! {}
            };
            quote! {
                .retry(Some(
                    ::rust_buildkite::CommandStepRetry::builder()
                        #automatic_tokens
                        #manual_tokens
                        .try_into()
                        .expect("invalid retry config")
                ))
            }
        } else {
            quote! {}
        };

        let plugins_tokens = if !self.plugins.is_empty() {
            let plugin_values: Vec<TokenStream2> = self.plugins.iter().map(|p| p.to_json_tokens()).collect();
            quote! {
                .plugins({
                    let __plugins_array = vec![#(#plugin_values),*];
                    Some(::rust_buildkite::Plugins::List(::rust_buildkite::PluginsList(__plugins_array.into_iter().map(|v| {
                        ::serde_json::from_value(v).expect("invalid plugin")
                    }).collect())))
                })
            }
        } else {
            quote! {}
        };

        let notify_tokens = if !self.notify.is_empty() {
            let notify_values: Vec<TokenStream2> = self.notify.iter().map(|n| n.to_json_tokens()).collect();
            quote! {
                .notify({
                    let __notify_array = vec![#(#notify_values),*];
                    Some(::rust_buildkite::CommandStepNotify(__notify_array.into_iter().map(|v| {
                        ::serde_json::from_value(v).expect("invalid notify")
                    }).collect()))
                })
            }
        } else {
            quote! {}
        };

        let matrix_tokens = if let Some(matrix) = &self.matrix {
            let matrix_json = matrix.to_json_tokens();
            quote! {
                .matrix({
                    let __matrix_value = #matrix_json;
                    Some(::serde_json::from_value(__matrix_value).expect("invalid matrix"))
                })
            }
        } else {
            quote! {}
        };

        let concurrency_tokens = if let Some(c) = &self.concurrency {
            quote! { .concurrency(Some(#c)) }
        } else {
            quote! {}
        };

        let concurrency_group_tokens = if let Some(group) = &self.concurrency_group {
            quote! { .concurrency_group(Some(#group.to_string())) }
        } else {
            quote! {}
        };

        let skip_tokens = match &self.skip {
            Some(SkipValue::Bool(true)) => {
                quote! { .skip(Some(::rust_buildkite::Skip::Boolean(true))) }
            }
            Some(SkipValue::Bool(false)) => {
                quote! { .skip(Some(::rust_buildkite::Skip::Boolean(false))) }
            }
            Some(SkipValue::Reason(reason)) => {
                quote! { .skip(Some(::rust_buildkite::Skip::String(#reason.parse().expect("invalid skip reason")))) }
            }
            None => quote! {},
        };

        let priority_tokens = if let Some(p) = &self.priority {
            quote! { .priority(Some(::rust_buildkite::Priority(#p))) }
        } else {
            quote! {}
        };

        let allow_dependency_failure_tokens = if self.allow_dependency_failure {
            quote! { .allow_dependency_failure(Some(::rust_buildkite::AllowDependencyFailure::Boolean(true))) }
        } else {
            quote! {}
        };

        quote! {
            ::rust_buildkite::GroupStepsItem::CommandStep(
                ::rust_buildkite::CommandStep::builder()
                    .command(Some(::rust_buildkite::CommandStepCommand::String(#cmd.to_string())))
                    #label_tokens
                    #key_tokens
                    #depends_on_tokens
                    #timeout_tokens
                    #soft_fail_tokens
                    #parallelism_tokens
                    #artifact_tokens
                    #env_tokens
                    #agents_tokens
                    #branches_tokens
                    #if_tokens
                    #cache_tokens
                    #retry_tokens
                    #plugins_tokens
                    #notify_tokens
                    #matrix_tokens
                    #concurrency_tokens
                    #concurrency_group_tokens
                    #skip_tokens
                    #priority_tokens
                    #allow_dependency_failure_tokens
                    .try_into()
                    .expect("command step construction failed")
            )
        }
    }
}

struct BlockStepDef {
    prompt: Option<LitStr>,
    key: Option<(String, proc_macro2::Span)>,
    depends_on: Vec<(String, proc_macro2::Span)>,
    fields: Vec<FieldDef>,
    allowed_teams: Vec<String>,
    blocked_state: Option<String>,
    branches: Vec<LitStr>,
    if_condition: Option<LitStr>,
    prompt_text: Option<LitStr>,
    allow_dependency_failure: bool,
}

impl BlockStepDef {
    fn new(prompt: LitStr) -> Self {
        Self {
            prompt: Some(prompt),
            key: None,
            depends_on: Vec::new(),
            fields: Vec::new(),
            allowed_teams: Vec::new(),
            blocked_state: None,
            branches: Vec::new(),
            if_condition: None,
            prompt_text: None,
            allow_dependency_failure: false,
        }
    }

    fn new_empty() -> Self {
        Self {
            prompt: None,
            key: None,
            depends_on: Vec::new(),
            fields: Vec::new(),
            allowed_teams: Vec::new(),
            blocked_state: None,
            branches: Vec::new(),
            if_condition: None,
            prompt_text: None,
            allow_dependency_failure: false,
        }
    }

    fn to_tokens_inner(&self) -> TokenStream2 {
        let prompt = self.prompt.as_ref().expect("block prompt must be set");

        let key_tokens = if let Some((key, _)) = &self.key {
            quote! { .key(Some(#key.to_string().try_into().expect("invalid key"))) }
        } else {
            quote! {}
        };

        let depends_on_tokens = if !self.depends_on.is_empty() {
            let deps: Vec<_> = self.depends_on.iter().map(|(d, _)| d).collect();
            quote! {
                .depends_on(Some(::rust_buildkite::DependsOn::DependsOnList(
                    ::rust_buildkite::DependsOnList(vec![
                        #(::rust_buildkite::DependsOnListItem::String(#deps.to_string())),*
                    ])
                )))
            }
        } else {
            quote! {}
        };

        let fields_tokens = if !self.fields.is_empty() {
            let field_items: Vec<TokenStream2> = self.fields.iter().map(|f| f.to_tokens_inner()).collect();
            quote! {
                .fields(Some(::rust_buildkite::Fields(vec![
                    #(#field_items),*
                ])))
            }
        } else {
            quote! {}
        };

        let allowed_teams_tokens = if !self.allowed_teams.is_empty() {
            let teams = &self.allowed_teams;
            quote! {
                .allowed_teams(Some(::rust_buildkite::AllowedTeams::Array(vec![
                    #(#teams.to_string()),*
                ])))
            }
        } else {
            quote! {}
        };

        let blocked_state_tokens = if let Some(state) = &self.blocked_state {
            quote! {
                .blocked_state(match #state {
                    "passed" => ::rust_buildkite::BlockStepBlockedState::Passed,
                    "failed" => ::rust_buildkite::BlockStepBlockedState::Failed,
                    "running" => ::rust_buildkite::BlockStepBlockedState::Running,
                    _ => ::rust_buildkite::BlockStepBlockedState::Passed,
                })
            }
        } else {
            quote! {}
        };

        let branches_tokens = if !self.branches.is_empty() {
            let branches = &self.branches;
            quote! {
                .branches(Some(::rust_buildkite::Branches::Array(vec![
                    #(#branches.to_string()),*
                ])))
            }
        } else {
            quote! {}
        };

        let if_tokens = if let Some(condition) = &self.if_condition {
            quote! { .if_(Some(::rust_buildkite::If(#condition.to_string()))) }
        } else {
            quote! {}
        };

        let prompt_tokens = if let Some(p) = &self.prompt_text {
            quote! { .prompt(Some(::rust_buildkite::Prompt(#p.to_string()))) }
        } else {
            quote! {}
        };

        let allow_dependency_failure_tokens = if self.allow_dependency_failure {
            quote! { .allow_dependency_failure(Some(::rust_buildkite::AllowDependencyFailure::Boolean(true))) }
        } else {
            quote! {}
        };

        quote! {
            ::rust_buildkite::PipelineStepsItem::BlockStep(
                ::rust_buildkite::BlockStep::builder()
                    .block(Some(#prompt.to_string()))
                    #key_tokens
                    #depends_on_tokens
                    #fields_tokens
                    #allowed_teams_tokens
                    #blocked_state_tokens
                    #branches_tokens
                    #if_tokens
                    #prompt_tokens
                    #allow_dependency_failure_tokens
                    .try_into()
                    .expect("block step construction failed")
            )
        }
    }

    fn to_group_step_tokens(&self) -> TokenStream2 {
        let prompt = self.prompt.as_ref().expect("block prompt must be set");

        let key_tokens = if let Some((key, _)) = &self.key {
            quote! { .key(Some(#key.to_string().try_into().expect("invalid key"))) }
        } else {
            quote! {}
        };

        let depends_on_tokens = if !self.depends_on.is_empty() {
            let deps: Vec<_> = self.depends_on.iter().map(|(d, _)| d).collect();
            quote! {
                .depends_on(Some(::rust_buildkite::DependsOn::DependsOnList(
                    ::rust_buildkite::DependsOnList(vec![
                        #(::rust_buildkite::DependsOnListItem::String(#deps.to_string())),*
                    ])
                )))
            }
        } else {
            quote! {}
        };

        let fields_tokens = if !self.fields.is_empty() {
            let field_items: Vec<TokenStream2> = self.fields.iter().map(|f| f.to_tokens_inner()).collect();
            quote! {
                .fields(Some(::rust_buildkite::Fields(vec![
                    #(#field_items),*
                ])))
            }
        } else {
            quote! {}
        };

        let allowed_teams_tokens = if !self.allowed_teams.is_empty() {
            let teams = &self.allowed_teams;
            quote! {
                .allowed_teams(Some(::rust_buildkite::AllowedTeams::Array(vec![
                    #(#teams.to_string()),*
                ])))
            }
        } else {
            quote! {}
        };

        let blocked_state_tokens = if let Some(state) = &self.blocked_state {
            quote! {
                .blocked_state(match #state {
                    "passed" => ::rust_buildkite::BlockStepBlockedState::Passed,
                    "failed" => ::rust_buildkite::BlockStepBlockedState::Failed,
                    "running" => ::rust_buildkite::BlockStepBlockedState::Running,
                    _ => ::rust_buildkite::BlockStepBlockedState::Passed,
                })
            }
        } else {
            quote! {}
        };

        let branches_tokens = if !self.branches.is_empty() {
            let branches = &self.branches;
            quote! {
                .branches(Some(::rust_buildkite::Branches::Array(vec![
                    #(#branches.to_string()),*
                ])))
            }
        } else {
            quote! {}
        };

        let if_tokens = if let Some(condition) = &self.if_condition {
            quote! { .if_(Some(::rust_buildkite::If(#condition.to_string()))) }
        } else {
            quote! {}
        };

        let prompt_tokens = if let Some(p) = &self.prompt_text {
            quote! { .prompt(Some(::rust_buildkite::Prompt(#p.to_string()))) }
        } else {
            quote! {}
        };

        let allow_dependency_failure_tokens = if self.allow_dependency_failure {
            quote! { .allow_dependency_failure(Some(::rust_buildkite::AllowDependencyFailure::Boolean(true))) }
        } else {
            quote! {}
        };

        quote! {
            ::rust_buildkite::GroupStepsItem::BlockStep(
                ::rust_buildkite::BlockStep::builder()
                    .block(Some(#prompt.to_string()))
                    #key_tokens
                    #depends_on_tokens
                    #fields_tokens
                    #allowed_teams_tokens
                    #blocked_state_tokens
                    #branches_tokens
                    #if_tokens
                    #prompt_tokens
                    #allow_dependency_failure_tokens
                    .try_into()
                    .expect("block step construction failed")
            )
        }
    }
}

struct InputStepDef {
    prompt: Option<LitStr>,
    key: Option<(String, proc_macro2::Span)>,
    depends_on: Vec<(String, proc_macro2::Span)>,
    fields: Vec<FieldDef>,
    allowed_teams: Vec<String>,
    blocked_state: Option<String>,
    branches: Vec<LitStr>,
    if_condition: Option<LitStr>,
    prompt_text: Option<LitStr>,
    allow_dependency_failure: bool,
}

impl InputStepDef {
    fn new(prompt: LitStr) -> Self {
        Self {
            prompt: Some(prompt),
            key: None,
            depends_on: Vec::new(),
            fields: Vec::new(),
            allowed_teams: Vec::new(),
            blocked_state: None,
            branches: Vec::new(),
            if_condition: None,
            prompt_text: None,
            allow_dependency_failure: false,
        }
    }

    fn new_empty() -> Self {
        Self {
            prompt: None,
            key: None,
            depends_on: Vec::new(),
            fields: Vec::new(),
            allowed_teams: Vec::new(),
            blocked_state: None,
            branches: Vec::new(),
            if_condition: None,
            prompt_text: None,
            allow_dependency_failure: false,
        }
    }

    fn to_tokens_inner(&self) -> TokenStream2 {
        let prompt = self.prompt.as_ref().expect("input prompt must be set");

        let key_tokens = if let Some((key, _)) = &self.key {
            quote! { .key(Some(#key.to_string().try_into().expect("invalid key"))) }
        } else {
            quote! {}
        };

        let depends_on_tokens = if !self.depends_on.is_empty() {
            let deps: Vec<_> = self.depends_on.iter().map(|(d, _)| d).collect();
            quote! {
                .depends_on(Some(::rust_buildkite::DependsOn::DependsOnList(
                    ::rust_buildkite::DependsOnList(vec![
                        #(::rust_buildkite::DependsOnListItem::String(#deps.to_string())),*
                    ])
                )))
            }
        } else {
            quote! {}
        };

        let fields_tokens = if !self.fields.is_empty() {
            let field_items: Vec<TokenStream2> = self.fields.iter().map(|f| f.to_tokens_inner()).collect();
            quote! {
                .fields(Some(::rust_buildkite::Fields(vec![
                    #(#field_items),*
                ])))
            }
        } else {
            quote! {}
        };

        let allowed_teams_tokens = if !self.allowed_teams.is_empty() {
            let teams = &self.allowed_teams;
            quote! {
                .allowed_teams(Some(::rust_buildkite::AllowedTeams::Array(vec![
                    #(#teams.to_string()),*
                ])))
            }
        } else {
            quote! {}
        };

        let blocked_state_tokens = if let Some(state) = &self.blocked_state {
            quote! {
                .blocked_state(match #state {
                    "passed" => ::rust_buildkite::InputStepBlockedState::Passed,
                    "failed" => ::rust_buildkite::InputStepBlockedState::Failed,
                    "running" => ::rust_buildkite::InputStepBlockedState::Running,
                    _ => ::rust_buildkite::InputStepBlockedState::Passed,
                })
            }
        } else {
            quote! {}
        };

        let branches_tokens = if !self.branches.is_empty() {
            let branches = &self.branches;
            quote! {
                .branches(Some(::rust_buildkite::Branches::Array(vec![
                    #(#branches.to_string()),*
                ])))
            }
        } else {
            quote! {}
        };

        let if_tokens = if let Some(condition) = &self.if_condition {
            quote! { .if_(Some(::rust_buildkite::If(#condition.to_string()))) }
        } else {
            quote! {}
        };

        let prompt_tokens = if let Some(p) = &self.prompt_text {
            quote! { .prompt(Some(::rust_buildkite::Prompt(#p.to_string()))) }
        } else {
            quote! {}
        };

        let allow_dependency_failure_tokens = if self.allow_dependency_failure {
            quote! { .allow_dependency_failure(Some(::rust_buildkite::AllowDependencyFailure::Boolean(true))) }
        } else {
            quote! {}
        };

        quote! {
            ::rust_buildkite::PipelineStepsItem::InputStep(
                ::rust_buildkite::InputStep::builder()
                    .input(Some(#prompt.to_string()))
                    #key_tokens
                    #depends_on_tokens
                    #fields_tokens
                    #allowed_teams_tokens
                    #blocked_state_tokens
                    #branches_tokens
                    #if_tokens
                    #prompt_tokens
                    #allow_dependency_failure_tokens
                    .try_into()
                    .expect("input step construction failed")
            )
        }
    }

    fn to_group_step_tokens(&self) -> TokenStream2 {
        let prompt = self.prompt.as_ref().expect("input prompt must be set");

        let key_tokens = if let Some((key, _)) = &self.key {
            quote! { .key(Some(#key.to_string().try_into().expect("invalid key"))) }
        } else {
            quote! {}
        };

        let depends_on_tokens = if !self.depends_on.is_empty() {
            let deps: Vec<_> = self.depends_on.iter().map(|(d, _)| d).collect();
            quote! {
                .depends_on(Some(::rust_buildkite::DependsOn::DependsOnList(
                    ::rust_buildkite::DependsOnList(vec![
                        #(::rust_buildkite::DependsOnListItem::String(#deps.to_string())),*
                    ])
                )))
            }
        } else {
            quote! {}
        };

        let fields_tokens = if !self.fields.is_empty() {
            let field_items: Vec<TokenStream2> = self.fields.iter().map(|f| f.to_tokens_inner()).collect();
            quote! {
                .fields(Some(::rust_buildkite::Fields(vec![
                    #(#field_items),*
                ])))
            }
        } else {
            quote! {}
        };

        let allowed_teams_tokens = if !self.allowed_teams.is_empty() {
            let teams = &self.allowed_teams;
            quote! {
                .allowed_teams(Some(::rust_buildkite::AllowedTeams::Array(vec![
                    #(#teams.to_string()),*
                ])))
            }
        } else {
            quote! {}
        };

        let blocked_state_tokens = if let Some(state) = &self.blocked_state {
            quote! {
                .blocked_state(match #state {
                    "passed" => ::rust_buildkite::InputStepBlockedState::Passed,
                    "failed" => ::rust_buildkite::InputStepBlockedState::Failed,
                    "running" => ::rust_buildkite::InputStepBlockedState::Running,
                    _ => ::rust_buildkite::InputStepBlockedState::Passed,
                })
            }
        } else {
            quote! {}
        };

        let branches_tokens = if !self.branches.is_empty() {
            let branches = &self.branches;
            quote! {
                .branches(Some(::rust_buildkite::Branches::Array(vec![
                    #(#branches.to_string()),*
                ])))
            }
        } else {
            quote! {}
        };

        let if_tokens = if let Some(condition) = &self.if_condition {
            quote! { .if_(Some(::rust_buildkite::If(#condition.to_string()))) }
        } else {
            quote! {}
        };

        let prompt_tokens = if let Some(p) = &self.prompt_text {
            quote! { .prompt(Some(::rust_buildkite::Prompt(#p.to_string()))) }
        } else {
            quote! {}
        };

        let allow_dependency_failure_tokens = if self.allow_dependency_failure {
            quote! { .allow_dependency_failure(Some(::rust_buildkite::AllowDependencyFailure::Boolean(true))) }
        } else {
            quote! {}
        };

        quote! {
            ::rust_buildkite::GroupStepsItem::InputStep(
                ::rust_buildkite::InputStep::builder()
                    .input(Some(#prompt.to_string()))
                    #key_tokens
                    #depends_on_tokens
                    #fields_tokens
                    #allowed_teams_tokens
                    #blocked_state_tokens
                    #branches_tokens
                    #if_tokens
                    #prompt_tokens
                    #allow_dependency_failure_tokens
                    .try_into()
                    .expect("input step construction failed")
            )
        }
    }
}

struct TriggerStepDef {
    pipeline: Option<LitStr>,
    label: Option<LitStr>,
    key: Option<(String, proc_macro2::Span)>,
    depends_on: Vec<(String, proc_macro2::Span)>,
    async_trigger: bool,
    build: Option<TriggerBuildConfig>,
    branches: Vec<LitStr>,
    if_condition: Option<LitStr>,
    skip: Option<SkipValue>,
    soft_fail: bool,
    allow_dependency_failure: bool,
}

/// Build configuration for trigger step
#[derive(Clone, Default)]
struct TriggerBuildConfig {
    branch: Option<String>,
    commit: Option<String>,
    message: Option<String>,
    env: Vec<(String, String)>,
    meta_data: Vec<(String, String)>,
}

impl TriggerStepDef {
    fn new(pipeline: LitStr) -> Self {
        Self {
            pipeline: Some(pipeline),
            label: None,
            key: None,
            depends_on: Vec::new(),
            async_trigger: false,
            build: None,
            branches: Vec::new(),
            if_condition: None,
            skip: None,
            soft_fail: false,
            allow_dependency_failure: false,
        }
    }

    fn new_empty() -> Self {
        Self {
            pipeline: None,
            label: None,
            key: None,
            depends_on: Vec::new(),
            async_trigger: false,
            build: None,
            branches: Vec::new(),
            if_condition: None,
            skip: None,
            soft_fail: false,
            allow_dependency_failure: false,
        }
    }

    fn to_tokens_inner(&self) -> TokenStream2 {
        let pipeline = self.pipeline.as_ref().expect("trigger pipeline must be set");

        let label_tokens = if let Some(label) = &self.label {
            quote! { .label(Some(::rust_buildkite::Label(#label.to_string()))) }
        } else {
            quote! {}
        };

        let key_tokens = if let Some((key, _)) = &self.key {
            quote! { .key(Some(#key.to_string().try_into().expect("invalid key"))) }
        } else {
            quote! {}
        };

        let depends_on_tokens = if !self.depends_on.is_empty() {
            let deps: Vec<_> = self.depends_on.iter().map(|(d, _)| d).collect();
            quote! {
                .depends_on(Some(::rust_buildkite::DependsOn::DependsOnList(
                    ::rust_buildkite::DependsOnList(vec![
                        #(::rust_buildkite::DependsOnListItem::String(#deps.to_string())),*
                    ])
                )))
            }
        } else {
            quote! {}
        };

        let async_tokens = if self.async_trigger {
            quote! { .async_(::rust_buildkite::TriggerStepAsync::Boolean(true)) }
        } else {
            quote! {}
        };

        let build_tokens = if let Some(build) = &self.build {
            let branch_tokens = if let Some(b) = &build.branch {
                quote! { .branch(#b.to_string()) }
            } else {
                quote! {}
            };
            let commit_tokens = if let Some(c) = &build.commit {
                quote! { .commit(#c.to_string()) }
            } else {
                quote! {}
            };
            let message_tokens = if let Some(m) = &build.message {
                quote! { .message(#m.to_string()) }
            } else {
                quote! {}
            };
            let env_tokens = if !build.env.is_empty() {
                let env_inserts: Vec<TokenStream2> = build.env.iter().map(|(k, v)| {
                    quote! { __build_env.insert(#k.to_string(), ::serde_json::Value::String(#v.to_string())); }
                }).collect();
                quote! {
                    .env({
                        let mut __build_env = ::serde_json::Map::new();
                        #(#env_inserts)*
                        Some(::rust_buildkite::Env(__build_env))
                    })
                }
            } else {
                quote! {}
            };
            let meta_data_tokens = if !build.meta_data.is_empty() {
                let md_inserts: Vec<TokenStream2> = build.meta_data.iter().map(|(k, v)| {
                    quote! { __build_meta.insert(#k.to_string(), ::serde_json::Value::String(#v.to_string())); }
                }).collect();
                quote! {
                    .meta_data({
                        let mut __build_meta = ::serde_json::Map::new();
                        #(#md_inserts)*
                        __build_meta
                    })
                }
            } else {
                quote! {}
            };
            quote! {
                .build(Some(
                    ::rust_buildkite::TriggerStepBuild::builder()
                        #branch_tokens
                        #commit_tokens
                        #message_tokens
                        #env_tokens
                        #meta_data_tokens
                        .try_into()
                        .expect("build config construction failed")
                ))
            }
        } else {
            quote! {}
        };

        let branches_tokens = if !self.branches.is_empty() {
            let branches = &self.branches;
            quote! {
                .branches(Some(::rust_buildkite::Branches::Array(vec![
                    #(#branches.to_string()),*
                ])))
            }
        } else {
            quote! {}
        };

        let if_tokens = if let Some(condition) = &self.if_condition {
            quote! { .if_(Some(::rust_buildkite::If(#condition.to_string()))) }
        } else {
            quote! {}
        };

        let skip_tokens = match &self.skip {
            Some(SkipValue::Bool(true)) => {
                quote! { .skip(Some(::rust_buildkite::Skip::Boolean(true))) }
            }
            Some(SkipValue::Bool(false)) => {
                quote! { .skip(Some(::rust_buildkite::Skip::Boolean(false))) }
            }
            Some(SkipValue::Reason(reason)) => {
                quote! { .skip(Some(::rust_buildkite::Skip::String(#reason.parse().expect("invalid skip reason")))) }
            }
            None => quote! {},
        };

        let soft_fail_tokens = if self.soft_fail {
            quote! { .soft_fail(Some(::rust_buildkite::SoftFail::Variant0(::rust_buildkite::SoftFailVariant0::Boolean(true)))) }
        } else {
            quote! {}
        };

        let allow_dependency_failure_tokens = if self.allow_dependency_failure {
            quote! { .allow_dependency_failure(Some(::rust_buildkite::AllowDependencyFailure::Boolean(true))) }
        } else {
            quote! {}
        };

        quote! {
            ::rust_buildkite::PipelineStepsItem::TriggerStep(
                ::rust_buildkite::TriggerStep::builder()
                    .trigger(#pipeline.to_string())
                    #label_tokens
                    #key_tokens
                    #depends_on_tokens
                    #async_tokens
                    #build_tokens
                    #branches_tokens
                    #if_tokens
                    #skip_tokens
                    #soft_fail_tokens
                    #allow_dependency_failure_tokens
                    .try_into()
                    .expect("trigger step construction failed")
            )
        }
    }

    fn to_group_step_tokens(&self) -> TokenStream2 {
        let pipeline = self.pipeline.as_ref().expect("trigger pipeline must be set");

        let label_tokens = if let Some(label) = &self.label {
            quote! { .label(Some(::rust_buildkite::Label(#label.to_string()))) }
        } else {
            quote! {}
        };

        let key_tokens = if let Some((key, _)) = &self.key {
            quote! { .key(Some(#key.to_string().try_into().expect("invalid key"))) }
        } else {
            quote! {}
        };

        let depends_on_tokens = if !self.depends_on.is_empty() {
            let deps: Vec<_> = self.depends_on.iter().map(|(d, _)| d).collect();
            quote! {
                .depends_on(Some(::rust_buildkite::DependsOn::DependsOnList(
                    ::rust_buildkite::DependsOnList(vec![
                        #(::rust_buildkite::DependsOnListItem::String(#deps.to_string())),*
                    ])
                )))
            }
        } else {
            quote! {}
        };

        let async_tokens = if self.async_trigger {
            quote! { .async_(Some(::rust_buildkite::TriggerStepAsync::Boolean(true))) }
        } else {
            quote! {}
        };

        let build_tokens = if let Some(build) = &self.build {
            let branch_tokens = if let Some(b) = &build.branch {
                quote! { .branch(#b.to_string()) }
            } else {
                quote! {}
            };
            let commit_tokens = if let Some(c) = &build.commit {
                quote! { .commit(#c.to_string()) }
            } else {
                quote! {}
            };
            let message_tokens = if let Some(m) = &build.message {
                quote! { .message(#m.to_string()) }
            } else {
                quote! {}
            };
            let env_tokens = if !build.env.is_empty() {
                let env_inserts: Vec<TokenStream2> = build.env.iter().map(|(k, v)| {
                    quote! { __build_env.insert(#k.to_string(), ::serde_json::Value::String(#v.to_string())); }
                }).collect();
                quote! {
                    .env({
                        let mut __build_env = ::serde_json::Map::new();
                        #(#env_inserts)*
                        Some(::rust_buildkite::Env(__build_env))
                    })
                }
            } else {
                quote! {}
            };
            let meta_data_tokens = if !build.meta_data.is_empty() {
                let md_inserts: Vec<TokenStream2> = build.meta_data.iter().map(|(k, v)| {
                    quote! { __build_meta.insert(#k.to_string(), ::serde_json::Value::String(#v.to_string())); }
                }).collect();
                quote! {
                    .meta_data({
                        let mut __build_meta = ::serde_json::Map::new();
                        #(#md_inserts)*
                        __build_meta
                    })
                }
            } else {
                quote! {}
            };
            quote! {
                .build(Some(
                    ::rust_buildkite::TriggerStepBuild::builder()
                        #branch_tokens
                        #commit_tokens
                        #message_tokens
                        #env_tokens
                        #meta_data_tokens
                        .try_into()
                        .expect("build config construction failed")
                ))
            }
        } else {
            quote! {}
        };

        let branches_tokens = if !self.branches.is_empty() {
            let branches = &self.branches;
            quote! {
                .branches(Some(::rust_buildkite::Branches::Array(vec![
                    #(#branches.to_string()),*
                ])))
            }
        } else {
            quote! {}
        };

        let if_tokens = if let Some(condition) = &self.if_condition {
            quote! { .if_(Some(::rust_buildkite::If(#condition.to_string()))) }
        } else {
            quote! {}
        };

        let skip_tokens = match &self.skip {
            Some(SkipValue::Bool(true)) => {
                quote! { .skip(Some(::rust_buildkite::Skip::Boolean(true))) }
            }
            Some(SkipValue::Bool(false)) => {
                quote! { .skip(Some(::rust_buildkite::Skip::Boolean(false))) }
            }
            Some(SkipValue::Reason(reason)) => {
                quote! { .skip(Some(::rust_buildkite::Skip::String(#reason.parse().expect("invalid skip reason")))) }
            }
            None => quote! {},
        };

        let soft_fail_tokens = if self.soft_fail {
            quote! { .soft_fail(Some(::rust_buildkite::SoftFail::Variant0(::rust_buildkite::SoftFailVariant0::Boolean(true)))) }
        } else {
            quote! {}
        };

        let allow_dependency_failure_tokens = if self.allow_dependency_failure {
            quote! { .allow_dependency_failure(Some(::rust_buildkite::AllowDependencyFailure::Boolean(true))) }
        } else {
            quote! {}
        };

        quote! {
            ::rust_buildkite::GroupStepsItem::TriggerStep(
                ::rust_buildkite::TriggerStep::builder()
                    .trigger(#pipeline.to_string())
                    #label_tokens
                    #key_tokens
                    #depends_on_tokens
                    #async_tokens
                    #build_tokens
                    #branches_tokens
                    #if_tokens
                    #skip_tokens
                    #soft_fail_tokens
                    #allow_dependency_failure_tokens
                    .try_into()
                    .expect("trigger step construction failed")
            )
        }
    }
}

struct GroupStepDef {
    label: Option<LitStr>,
    key: Option<(String, proc_macro2::Span)>,
    depends_on: Vec<(String, proc_macro2::Span)>,
    steps: Vec<StepDef>,
    if_condition: Option<LitStr>,
    skip: Option<SkipValue>,
    notify: Vec<NestedValue>,
    allow_dependency_failure: bool,
}

impl GroupStepDef {
    fn new(label: LitStr) -> Self {
        Self {
            label: Some(label),
            key: None,
            depends_on: Vec::new(),
            steps: Vec::new(),
            if_condition: None,
            skip: None,
            notify: Vec::new(),
            allow_dependency_failure: false,
        }
    }

    fn new_empty() -> Self {
        Self {
            label: None,
            key: None,
            depends_on: Vec::new(),
            steps: Vec::new(),
            if_condition: None,
            skip: None,
            notify: Vec::new(),
            allow_dependency_failure: false,
        }
    }

    fn to_tokens_inner(&self) -> TokenStream2 {
        let label = self.label.as_ref().expect("group label must be set");

        let key_tokens = if let Some((key, _)) = &self.key {
            quote! { .key(Some(#key.to_string().try_into().expect("invalid key"))) }
        } else {
            quote! {}
        };

        let depends_on_tokens = if !self.depends_on.is_empty() {
            let deps: Vec<_> = self.depends_on.iter().map(|(d, _)| d).collect();
            quote! {
                .depends_on(Some(::rust_buildkite::DependsOn::DependsOnList(
                    ::rust_buildkite::DependsOnList(vec![
                        #(::rust_buildkite::DependsOnListItem::String(#deps.to_string())),*
                    ])
                )))
            }
        } else {
            quote! {}
        };

        let nested_steps: Vec<TokenStream2> = self.steps.iter().map(|s| s.to_group_step_tokens()).collect();
        let steps_tokens = quote! {
            .steps(::rust_buildkite::GroupSteps(vec![
                #(#nested_steps),*
            ]))
        };

        let if_tokens = if let Some(condition) = &self.if_condition {
            quote! { .if_(Some(::rust_buildkite::If(#condition.to_string()))) }
        } else {
            quote! {}
        };

        let skip_tokens = match &self.skip {
            Some(SkipValue::Bool(true)) => {
                quote! { .skip(Some(::rust_buildkite::Skip::Boolean(true))) }
            }
            Some(SkipValue::Bool(false)) => {
                quote! { .skip(Some(::rust_buildkite::Skip::Boolean(false))) }
            }
            Some(SkipValue::Reason(reason)) => {
                quote! { .skip(Some(::rust_buildkite::Skip::String(#reason.parse().expect("invalid skip reason")))) }
            }
            None => quote! {},
        };

        let notify_tokens = if !self.notify.is_empty() {
            let notify_values: Vec<TokenStream2> = self.notify.iter().map(|n| n.to_json_tokens()).collect();
            quote! {
                .notify({
                    let __notify_array = vec![#(#notify_values),*];
                    Some(::rust_buildkite::BuildNotify(__notify_array.into_iter().map(|v| {
                        ::serde_json::from_value(v).expect("invalid notify")
                    }).collect()))
                })
            }
        } else {
            quote! {}
        };

        let allow_dependency_failure_tokens = if self.allow_dependency_failure {
            quote! { .allow_dependency_failure(Some(::rust_buildkite::AllowDependencyFailure::Boolean(true))) }
        } else {
            quote! {}
        };

        quote! {
            ::rust_buildkite::PipelineStepsItem::GroupStep(
                ::rust_buildkite::GroupStep::builder()
                    .group(Some(#label.to_string()))
                    #key_tokens
                    #depends_on_tokens
                    #steps_tokens
                    #if_tokens
                    #skip_tokens
                    #notify_tokens
                    #allow_dependency_failure_tokens
                    .try_into()
                    .expect("group step construction failed")
            )
        }
    }
}
