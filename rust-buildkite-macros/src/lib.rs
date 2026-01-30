//! Proc macros for type-safe Buildkite pipeline DSL
//!
//! This crate provides the `pipeline!` macro for declaratively defining
//! Buildkite pipelines with compile-time validation.
//!
//! Shell commands are validated using [bashrs](https://docs.rs/bashrs) for
//! proper parsing and linting at compile time.

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


const SHELL_BUILTINS: &[&str] = &[
    // nb: POSIX builtins
    ".", ":", "[", "alias", "bg", "cd", "command", "eval", "exec", "exit",
    "export", "fc", "fg", "getopts", "hash", "jobs", "kill", "newgrp",
    "pwd", "read", "readonly", "return", "set", "shift", "source", "test",
    "times", "trap", "type", "ulimit", "umask", "unalias", "unset", "wait",
    // nb: bash builtins because we always force bash. need to extend this if 
    // support for other shells is added.
    "bind", "builtin", "caller", "compgen", "complete", "compopt", "declare",
    "dirs", "disown", "enable", "help", "history", "let", "local", "logout",
    "mapfile", "popd", "printf", "pushd", "readarray", "shopt", "suspend",
    "typeset",
];

/// Expand known runtime_env keywords to their literal values.
/// These are recognized by the macro and expanded inline.
fn expand_known_env_list(ident: &str) -> Option<&'static [&'static str]> {
    match ident {
        "SHELL_ENV" => Some(&[
            "HOME", "PATH", "USER", "SHELL", "PWD", "OLDPWD", "TERM", "HOSTNAME", 
            "LANG", "LC_ALL", "TZ", "TMPDIR",
        ]),
        "BUILDKITE_ENV" => Some(&[
            "BUILDKITE", "BUILDKITE_AGENT_NAME", "BUILDKITE_BRANCH", "BUILDKITE_BUILD_ID",
            "BUILDKITE_BUILD_NUMBER", "BUILDKITE_BUILD_URL", "BUILDKITE_COMMIT", 
            "BUILDKITE_JOB_ID", "BUILDKITE_LABEL", "BUILDKITE_MESSAGE",
            "BUILDKITE_ORGANIZATION_SLUG", "BUILDKITE_PIPELINE_ID", "BUILDKITE_PIPELINE_SLUG",
            "BUILDKITE_PULL_REQUEST", "BUILDKITE_PULL_REQUEST_BASE_BRANCH", "BUILDKITE_PULL_REQUEST_REPO",
            "BUILDKITE_REPO", "BUILDKITE_SOURCE", "BUILDKITE_STEP_KEY", "BUILDKITE_TAG",
            "BUILDKITE_TRIGGERED_FROM_BUILD_ID",
        ]),
        "CI_ENV" => Some(&["CI", "CI_BUILD_NUMBER", "CI_COMMIT_SHA", "CI_BRANCH"]),
        _ => None,
    }
}

/// Discover all environment variables on the host machine at compile time.
/// This provides the default runtime_env list.
fn discover_host_env_vars() -> HashSet<String> {
    std::env::vars().map(|(k, _)| k).collect()
}

/// Discover all executable commands in the host machine's PATH at compile time,
/// plus shell builtins. This provides the default allowed_commands list.
fn discover_host_path_commands() -> HashSet<String> {
    use std::fs;
    #[cfg(unix)]
    use std::os::unix::fs::PermissionsExt;
    
    let mut commands = HashSet::new();
    for builtin in SHELL_BUILTINS {
        commands.insert((*builtin).to_string());
    }
    
    if let Ok(path) = std::env::var("PATH") {
        for dir in path.split(':') {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        #[cfg(unix)]
                        {
                            if let Ok(metadata) = path.metadata() {
                                let mode = metadata.permissions().mode();
                                if mode & 0o111 != 0 {
                                    if let Some(name) = path.file_name() {
                                        if let Some(name_str) = name.to_str() {
                                            commands.insert(name_str.to_string());
                                        }
                                    }
                                }
                            }
                        }
                        #[cfg(not(unix))]
                        {
                            if let Some(name) = path.file_name() {
                                if let Some(name_str) = name.to_str() {
                                    commands.insert(name_str.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    commands
}

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
                    // Strip r# prefix for raw identifiers (e.g., r#if -> if)
                    strip_raw_ident(&ident.to_string()).to_string()
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
                quote! { ::rust_buildkite::serde_json::Value::String(#s.to_string()) }
            }
            NestedValue::Int(i) => {
                quote! { ::rust_buildkite::serde_json::Value::Number(::rust_buildkite::serde_json::Number::from(#i)) }
            }
            NestedValue::Bool(b) => {
                quote! { ::rust_buildkite::serde_json::Value::Bool(#b) }
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
                        let mut __obj = ::rust_buildkite::serde_json::Map::new();
                        #(#inserts)*
                        ::rust_buildkite::serde_json::Value::Object(__obj)
                    }
                }
            }
            NestedValue::Array(items) => {
                let item_tokens: Vec<TokenStream2> = items.iter().map(|v| v.to_json_tokens()).collect();
                quote! {
                    ::rust_buildkite::serde_json::Value::Array(vec![#(#item_tokens),*])
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

enum RuntimeEnvItem {
    Literal(String),
    ConstRef(syn::Path),
}

#[derive(Clone)]
enum NotifyValue {
    Slack { channel: String, if_: Option<String> },
    Email { email: String, if_: Option<String> },
    Webhook { url: String, if_: Option<String> },
    Pagerduty { service: String, if_: Option<String> },
    GithubCommitStatus { context: Option<String>, if_: Option<String> },
    GithubCheck,
    Basecamp { url: String, if_: Option<String> },
}

impl NotifyValue {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        braced!(content in input);
        
        let first_key: Ident = content.parse()?;
        content.parse::<Token![:]>()?;
        
        match strip_raw_ident(&first_key.to_string()) {
            "slack" => {
                let channel: LitStr = content.parse()?;
                let if_ = Self::parse_optional_if(&content)?;
                Ok(NotifyValue::Slack { channel: channel.value(), if_ })
            }
            "email" => {
                let email: LitStr = content.parse()?;
                let if_ = Self::parse_optional_if(&content)?;
                Ok(NotifyValue::Email { email: email.value(), if_ })
            }
            "webhook" => {
                let url: LitStr = content.parse()?;
                let if_ = Self::parse_optional_if(&content)?;
                Ok(NotifyValue::Webhook { url: url.value(), if_ })
            }
            "pagerduty_change_event" => {
                let service: LitStr = content.parse()?;
                let if_ = Self::parse_optional_if(&content)?;
                Ok(NotifyValue::Pagerduty { service: service.value(), if_ })
            }
            "github_commit_status" => {
                let nested = NestedValue::parse(&content)?;
                let context = if let NestedValue::Object(pairs) = nested {
                    pairs.iter().find(|(k, _)| k == "context").and_then(|(_, v)| {
                        if let NestedValue::String(s) = v { Some(s.clone()) } else { None }
                    })
                } else {
                    None
                };
                let if_ = Self::parse_optional_if(&content)?;
                Ok(NotifyValue::GithubCommitStatus { context, if_ })
            }
            "github_check" => {
                let _ = NestedValue::parse(&content)?;
                Ok(NotifyValue::GithubCheck)
            }
            "basecamp_campfire" => {
                let url: LitStr = content.parse()?;
                let if_ = Self::parse_optional_if(&content)?;
                Ok(NotifyValue::Basecamp { url: url.value(), if_ })
            }
            other => Err(Error::new(first_key.span(), format!("unknown notify type: {}", other)))
        }
    }
    
    fn parse_optional_if(content: ParseStream) -> Result<Option<String>> {
        if content.peek(Token![,]) {
            content.parse::<Token![,]>()?;
            if content.peek(Ident) {
                let key: Ident = content.parse()?;
                if strip_raw_ident(&key.to_string()) == "if" {
                    content.parse::<Token![:]>()?;
                    let val: LitStr = content.parse()?;
                    if content.peek(Token![,]) {
                        content.parse::<Token![,]>()?;
                    }
                    return Ok(Some(val.value()));
                }
            }
        }
        Ok(None)
    }

    fn to_tokens(&self) -> TokenStream2 {
        match self {
            NotifyValue::Slack { channel, if_ } => {
                let if_tokens = match if_ {
                    Some(c) => quote! { Some(::rust_buildkite::If(#c.to_string())) },
                    None => quote! { None },
                };
                quote! {
                    ::rust_buildkite::BuildNotifyItem::Slack(::rust_buildkite::NotifySlack {
                        slack: Some(::rust_buildkite::NotifySlackSlack::String(#channel.to_string())),
                        if_: #if_tokens,
                    })
                }
            }
            NotifyValue::Email { email, if_ } => {
                let if_tokens = match if_ {
                    Some(c) => quote! { Some(::rust_buildkite::If(#c.to_string())) },
                    None => quote! { None },
                };
                quote! {
                    ::rust_buildkite::BuildNotifyItem::Email(::rust_buildkite::NotifyEmail {
                        email: Some(#email.to_string()),
                        if_: #if_tokens,
                    })
                }
            }
            NotifyValue::Webhook { url, if_ } => {
                let if_tokens = match if_ {
                    Some(c) => quote! { Some(::rust_buildkite::If(#c.to_string())) },
                    None => quote! { None },
                };
                quote! {
                    ::rust_buildkite::BuildNotifyItem::Webhook(::rust_buildkite::NotifyWebhook {
                        webhook: Some(#url.to_string()),
                        if_: #if_tokens,
                    })
                }
            }
            NotifyValue::Pagerduty { service, if_ } => {
                let if_tokens = match if_ {
                    Some(c) => quote! { Some(::rust_buildkite::If(#c.to_string())) },
                    None => quote! { None },
                };
                quote! {
                    ::rust_buildkite::BuildNotifyItem::Pagerduty(::rust_buildkite::NotifyPagerduty {
                        pagerduty_change_event: Some(#service.to_string()),
                        if_: #if_tokens,
                    })
                }
            }
            NotifyValue::GithubCommitStatus { context, if_ } => {
                let if_tokens = match if_ {
                    Some(c) => quote! { Some(::rust_buildkite::If(#c.to_string())) },
                    None => quote! { None },
                };
                let context_tokens = match context {
                    Some(c) => quote! { Some(::rust_buildkite::NotifyGithubCommitStatusGithubCommitStatus { context: Some(#c.to_string()) }) },
                    None => quote! { None },
                };
                quote! {
                    ::rust_buildkite::BuildNotifyItem::GithubCommitStatus(::rust_buildkite::NotifyGithubCommitStatus {
                        github_commit_status: #context_tokens,
                        if_: #if_tokens,
                    })
                }
            }
            NotifyValue::GithubCheck => {
                quote! {
                    ::rust_buildkite::BuildNotifyItem::GithubCheck(::rust_buildkite::NotifyGithubCheck {
                        github_check: None,
                    })
                }
            }
            NotifyValue::Basecamp { url, if_ } => {
                let if_tokens = match if_ {
                    Some(c) => quote! { Some(::rust_buildkite::If(#c.to_string())) },
                    None => quote! { None },
                };
                quote! {
                    ::rust_buildkite::BuildNotifyItem::Basecamp(::rust_buildkite::NotifyBasecamp {
                        basecamp_campfire: Some(#url.to_string()),
                        if_: #if_tokens,
                    })
                }
            }
        }
    }
}

#[derive(Clone)]
enum SecretsValue {
    Array(Vec<String>),
    Object(Vec<(String, String)>),
}

struct PipelineDef {
    allowed_commands: Option<Vec<(String, proc_macro2::Span)>>,
    additional_commands: Vec<String>,
    allow_missing_paths: Vec<String>,
    runtime_env: Option<Vec<RuntimeEnvItem>>,
    env: Option<Vec<(Ident, LitStr)>>,
    steps: Vec<StepDef>,
    agents: Vec<(String, String)>,
    notify: Vec<NotifyValue>,
    image: Option<String>,
    secrets: Option<SecretsValue>,
    priority: Option<i64>,
    default_plugins: Vec<NestedValue>,
}

impl Parse for PipelineDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut allowed_commands = None;
        let mut additional_commands = Vec::new();
        let mut allow_missing_paths = Vec::new();
        let mut runtime_env = None;
        let mut env = None;
        let mut steps = Vec::new();
        let mut agents = Vec::new();
        let mut notify = Vec::new();
        let mut image = None;
        let mut secrets = None;
        let mut priority = None;
        let mut default_plugins = Vec::new();

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![:]>()?;

            match key.to_string().as_str() {
                "allow_missing_paths" => {
                    let content;
                    bracketed!(content in input);
                    while !content.is_empty() {
                        let lit: LitStr = content.parse()?;
                        allow_missing_paths.push(lit.value());
                        if content.peek(Token![,]) {
                            content.parse::<Token![,]>()?;
                        }
                    }
                }
                "runtime_env" => {
                    let content;
                    bracketed!(content in input);
                    let mut vars = Vec::new();
                    while !content.is_empty() {
                        if content.peek(LitStr) {
                            let lit: LitStr = content.parse()?;
                            vars.push(RuntimeEnvItem::Literal(lit.value()));
                        } else {
                            let path: syn::Path = content.parse()?;
                            let ident_str = path.get_ident().map(|i| i.to_string());
                            
                            if let Some(ref name) = ident_str {
                                if let Some(known_vars) = expand_known_env_list(name) {
                                    for var in known_vars {
                                        vars.push(RuntimeEnvItem::Literal(var.to_string()));
                                    }
                                } else {
                                    vars.push(RuntimeEnvItem::ConstRef(path));
                                }
                            } else {
                                vars.push(RuntimeEnvItem::ConstRef(path));
                            }
                        }
                        if content.peek(Token![,]) {
                            content.parse::<Token![,]>()?;
                        }
                    }
                    runtime_env = Some(vars);
                }
                "allowed_commands" => {
                    let content;
                    bracketed!(content in input);
                    let mut commands = Vec::new();
                    while !content.is_empty() {
                        let cmd_span = content.span();
                        if content.peek(LitStr) {
                            let lit: LitStr = content.parse()?;
                            commands.push((lit.value(), cmd_span));
                        } else {
                            let mut cmd_name = String::new();
                            while !content.is_empty() && !content.peek(Token![,]) {
                                if content.peek(Token![.]) {
                                    content.parse::<Token![.]>()?;
                                    cmd_name.push('.');
                                } else if content.peek(Token![/]) {
                                    content.parse::<Token![/]>()?;
                                    cmd_name.push('/');
                                } else if content.peek(Token![-]) {
                                    content.parse::<Token![-]>()?;
                                    cmd_name.push('-');
                                } else if content.peek(Token![_]) {
                                    content.parse::<Token![_]>()?;
                                    cmd_name.push('_');
                                } else if content.peek(Ident) {
                                    let ident: Ident = content.parse()?;
                                    cmd_name.push_str(&ident.to_string());
                                } else if content.peek(syn::LitInt) {
                                    let lit: syn::LitInt = content.parse()?;
                                    cmd_name.push_str(&lit.to_string());
                                } else {
                                    break;
                                }
                            }
                            
                            if !cmd_name.is_empty() {
                                commands.push((cmd_name, cmd_span));
                            }
                        }
                        
                        if content.peek(Token![,]) {
                            content.parse::<Token![,]>()?;
                        }
                    }
                    allowed_commands = Some(commands);
                }
                "additional_commands" => {
                    let content;
                    bracketed!(content in input);
                    while !content.is_empty() {
                        if content.peek(LitStr) {
                            let lit: LitStr = content.parse()?;
                            additional_commands.push(lit.value());
                        } else {
                            let mut cmd_name = String::new();
                            while !content.is_empty() && !content.peek(Token![,]) {
                                if content.peek(Token![.]) {
                                    content.parse::<Token![.]>()?;
                                    cmd_name.push('.');
                                } else if content.peek(Token![/]) {
                                    content.parse::<Token![/]>()?;
                                    cmd_name.push('/');
                                } else if content.peek(Token![-]) {
                                    content.parse::<Token![-]>()?;
                                    cmd_name.push('-');
                                } else if content.peek(Token![_]) {
                                    content.parse::<Token![_]>()?;
                                    cmd_name.push('_');
                                } else if content.peek(Ident) {
                                    let ident: Ident = content.parse()?;
                                    cmd_name.push_str(&ident.to_string());
                                } else if content.peek(syn::LitInt) {
                                    let lit: syn::LitInt = content.parse()?;
                                    cmd_name.push_str(&lit.to_string());
                                } else {
                                    break;
                                }
                            }
                            if !cmd_name.is_empty() {
                                additional_commands.push(cmd_name);
                            }
                        }
                        if content.peek(Token![,]) {
                            content.parse::<Token![,]>()?;
                        }
                    }
                }
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
                "agents" => {
                    let content;
                    braced!(content in input);
                    while !content.is_empty() {
                        let k: Ident = content.parse()?;
                        content.parse::<Token![:]>()?;
                        let v: LitStr = content.parse()?;
                        agents.push((k.to_string(), v.value()));
                        if content.peek(Token![,]) {
                            content.parse::<Token![,]>()?;
                        }
                    }
                }
                "notify" => {
                    let content;
                    bracketed!(content in input);
                    while !content.is_empty() {
                        notify.push(NotifyValue::parse(&content)?);
                        if content.peek(Token![,]) {
                            content.parse::<Token![,]>()?;
                        }
                    }
                }
                "image" => {
                    let lit: LitStr = input.parse()?;
                    image = Some(lit.value());
                }
                "secrets" => {
                    if input.peek(syn::token::Bracket) {
                        let content;
                        bracketed!(content in input);
                        let mut items = Vec::new();
                        while !content.is_empty() {
                            let lit: LitStr = content.parse()?;
                            items.push(lit.value());
                            if content.peek(Token![,]) {
                                content.parse::<Token![,]>()?;
                            }
                        }
                        secrets = Some(SecretsValue::Array(items));
                    } else {
                        let content;
                        braced!(content in input);
                        let mut pairs = Vec::new();
                        while !content.is_empty() {
                            let k: Ident = content.parse()?;
                            content.parse::<Token![:]>()?;
                            let v: LitStr = content.parse()?;
                            pairs.push((k.to_string(), v.value()));
                            if content.peek(Token![,]) {
                                content.parse::<Token![,]>()?;
                            }
                        }
                        secrets = Some(SecretsValue::Object(pairs));
                    }
                }
                "priority" => {
                    let lit: syn::LitInt = input.parse()?;
                    priority = Some(lit.base10_parse()?);
                }
                "default_plugins" => {
                    let content;
                    bracketed!(content in input);
                    while !content.is_empty() {
                        let plugin = NestedValue::parse(&content)?;
                        default_plugins.push(plugin);
                        if content.peek(Token![,]) {
                            content.parse::<Token![,]>()?;
                        }
                    }
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

        Ok(PipelineDef {
            allowed_commands,
            additional_commands,
            allow_missing_paths,
            runtime_env,
            env,
            steps,
            agents,
            notify,
            image,
            secrets,
            priority,
            default_plugins,
        })
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
        let allow_missing: Vec<&str> = self.allow_missing_paths.iter().map(|s| s.as_str()).collect();
        self.validate_paths(&self.steps, &allow_missing)?;
        let mut allowed_names: HashSet<String> = if let Some(allowed) = &self.allowed_commands {
            allowed.iter().map(|(s, _)| s.clone()).collect()
        } else {
            discover_host_path_commands()
        };
        for cmd in &self.additional_commands {
            allowed_names.insert(cmd.clone());
        }
        let allowed_refs: HashSet<&str> = allowed_names.iter().map(|s| s.as_str()).collect();
        self.validate_commands(&self.steps, &allowed_refs)?;
        self.validate_env_vars(&self.steps)?;

        let step_tokens: Vec<TokenStream2> = self
            .steps
            .iter()
            .map(|s| s.to_tokens_with_default_plugins(&self.default_plugins))
            .collect();
        let env_tokens = if let Some(env_vars) = &self.env {
            let env_inserts: Vec<TokenStream2> = env_vars
                .iter()
                .map(|(k, v)| {
                    let key_str = k.to_string();
                    quote! {
                        __env_map.insert(
                            #key_str.to_string(),
                            ::rust_buildkite::serde_json::Value::String(#v.to_string())
                        );
                    }
                })
                .collect();

            quote! {
                {
                    let mut __env_map = ::rust_buildkite::serde_json::Map::new();
                    #(#env_inserts)*
                    Some(::rust_buildkite::Env(__env_map))
                }
            }
        } else {
            quote! { None }
        };

        // nb: code to "use" any const refs to suppress unused import warnings
        let const_ref_uses: Vec<TokenStream2> = if let Some(runtime_env) = &self.runtime_env {
            runtime_env.iter().filter_map(|item| {
                if let RuntimeEnvItem::ConstRef(path) = item {
                    Some(quote! { let _ = #path; })
                } else {
                    None
                }
            }).collect()
        } else {
            Vec::new()
        };

        let agents_tokens = if !self.agents.is_empty() {
            let inserts: Vec<TokenStream2> = self.agents.iter().map(|(k, v)| {
                quote! { __agents_map.insert(#k.to_string(), ::rust_buildkite::serde_json::Value::String(#v.to_string())); }
            }).collect();
            quote! {
                .agents({
                    let mut __agents_map = ::rust_buildkite::serde_json::Map::new();
                    #(#inserts)*
                    Some(::rust_buildkite::Agents::Object(::rust_buildkite::AgentsObject(__agents_map)))
                })
            }
        } else {
            quote! {}
        };

        let notify_tokens = if !self.notify.is_empty() {
            let items: Vec<TokenStream2> = self.notify.iter().map(|n| n.to_tokens()).collect();
            quote! {
                .notify(Some(::rust_buildkite::BuildNotify(vec![#(#items),*])))
            }
        } else {
            quote! {}
        };

        let image_tokens = match &self.image {
            Some(i) => quote! { .image(Some(::rust_buildkite::Image(#i.to_string()))) },
            None => quote! {},
        };

        let secrets_tokens = match &self.secrets {
            Some(SecretsValue::Array(items)) => {
                quote! {
                    .secrets(Some(::rust_buildkite::Secrets::Array(vec![#(#items.to_string()),*])))
                }
            }
            Some(SecretsValue::Object(pairs)) => {
                let inserts: Vec<TokenStream2> = pairs.iter().map(|(k, v)| {
                    quote! { __secrets_map.insert(#k.to_string(), #v.to_string()); }
                }).collect();
                quote! {
                    .secrets({
                        let mut __secrets_map = ::std::collections::HashMap::new();
                        #(#inserts)*
                        Some(::rust_buildkite::Secrets::Object(__secrets_map))
                    })
                }
            }
            None => quote! {},
        };

        let priority_tokens = match &self.priority {
            Some(p) => quote! { .priority(Some(::rust_buildkite::Priority(#p))) },
            None => quote! {},
        };

        Ok(quote! {
            {
                #(#const_ref_uses)*
                
                let __result: ::rust_buildkite::JsonSchemaForBuildkitePipelineConfigurationFiles = 
                    ::rust_buildkite::JsonSchemaForBuildkitePipelineConfigurationFiles::builder()
                        .steps(::rust_buildkite::PipelineSteps(vec![
                            #(#step_tokens),*
                        ]))
                        .env(#env_tokens)
                        #agents_tokens
                        #notify_tokens
                        #image_tokens
                        #secrets_tokens
                        #priority_tokens
                        .try_into()
                        .expect("pipeline construction failed");
                __result
            }
        })
    }

    /// Validate all command steps against the allowed commands list.
    /// When allowed_commands is set, the command name must be in the allowed list.
    /// Note: Raw strings are already rejected at parse time - cmd!() is always required.
    /// Note: Path-based commands (./script, /path/to/cmd) bypass allowlist - they're validated
    ///       separately by validate_paths() for existence.
    fn validate_commands(&self, steps: &[StepDef], allowed: &HashSet<&str>) -> Result<()> {
        for step in steps {
            match step {
                StepDef::Command(cmd_step) => {
                    if let Some((cmd_name, span)) = cmd_step.get_command_name() {
                        if cmd_name.starts_with('/') || cmd_name.starts_with("./") {
                            continue;
                        }
                        
                        if !allowed.contains(cmd_name.as_str()) {
                            let mut available: Vec<_> = allowed.iter().copied().collect();
                            available.sort();
                            return Err(Error::new(
                                span,
                                format!(
                                    "Command '{}' is not in allowed_commands list.\n\
                                     Available commands: {:?}\n\
                                     Add '{}' to allowed_commands or use a different command.",
                                    cmd_name, available, cmd_name
                                ),
                            ));
                        }
                    }
                }
                StepDef::Group(group) => {
                    self.validate_commands(&group.steps, allowed)?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// Validate that path-based commands (./script.sh, /usr/bin/env) exist at compile time.
    /// Paths in allow_missing are skipped (for runtime-only paths).
    fn validate_paths(&self, steps: &[StepDef], allow_missing: &[&str]) -> Result<()> {
        for step in steps {
            match step {
                StepDef::Command(cmd_step) => {
                    if let Some((cmd_name, span)) = cmd_step.get_command_name() {
                        if cmd_name.starts_with('/') || cmd_name.starts_with("./") {
                            if let Err(e) = CmdExpr::validate_path_exists(&cmd_name, allow_missing) {
                                return Err(Error::new(span, e));
                            }
                        }
                    }
                }
                StepDef::Group(group) => {
                    self.validate_paths(&group.steps, allow_missing)?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// Validate that environment variables used in commands are defined.
    /// Variables must be explicitly defined in: pipeline env block, step env, or runtime_env list.
    /// If runtime_env is not specified, defaults to host environment variables.
    fn validate_env_vars(&self, steps: &[StepDef]) -> Result<()> {
        let mut has_const_refs = false;
        
        let mut allowed_vars: HashSet<String> = HashSet::new();
        if let Some(env_vars) = &self.env {
            for (name, _) in env_vars {
                allowed_vars.insert(name.to_string());
            }
        }
        
        if let Some(runtime_env) = &self.runtime_env {
            for item in runtime_env {
                match item {
                    RuntimeEnvItem::Literal(name) => {
                        allowed_vars.insert(name.clone());
                    }
                    RuntimeEnvItem::ConstRef(_) => {
                        has_const_refs = true;
                    }
                }
            }
        } else {
            allowed_vars.extend(discover_host_env_vars());
        }
        
        if has_const_refs {
            return Ok(());
        }
        
        self.validate_env_vars_in_steps(steps, &allowed_vars)
    }
    
    fn validate_env_vars_in_steps(&self, steps: &[StepDef], allowed: &HashSet<String>) -> Result<()> {
        for step in steps {
            match step {
                StepDef::Command(cmd_step) => {
                    let mut step_allowed = allowed.clone();
                    for (name, _) in &cmd_step.env {
                        step_allowed.insert(name.clone());
                    }
                    
                    if let Some(cmd_value) = &cmd_step.command {
                        let span = cmd_value.span();
                        let undefined_vars = cmd_value.get_undefined_vars();
                        
                        for var in undefined_vars {
                            if !step_allowed.contains(var) {
                                return Err(Error::new(
                                    span,
                                    format!(
                                        "Environment variable '{}' is not defined.\n\
                                         Add it to pipeline env: env: {{ {}: \"value\" }}\n\
                                         Or allow it: runtime_env: [\"{}\"]",
                                        var, var, var
                                    ),
                                ));
                            }
                        }
                    }
                }
                StepDef::Group(group) => {
                    self.validate_env_vars_in_steps(&group.steps, allowed)?;
                }
                _ => {}
            }
        }
        Ok(())
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

    /// Parse command step with fluent syntax: command(cmd!("...")).method()
    /// Raw strings are not allowed - must use cmd!() for bashrs validation.
    fn parse_command_fluent(input: ParseStream) -> Result<Self> {
        let content;
        syn::parenthesized!(content in input);
        let mut step = if content.peek(Ident) {
            let ident: Ident = content.parse()?;
            if ident == "cmd" {
                content.parse::<Token![!]>()?;
                let cmd_content;
                syn::parenthesized!(cmd_content in content);
                let lit: LitStr = cmd_content.parse().map_err(|_| {
                    Error::new(
                        cmd_content.span(),
                        "cmd! requires a string literal, e.g., cmd!(\"npm install\")"
                    )
                })?;
                let cmd_expr = CmdExpr::from_lit_str(&lit)?;
                CommandStepDef::new_with_cmd(cmd_expr)
            } else {
                return Err(Error::new(
                    ident.span(),
                    format!("expected cmd!(\"...\"), got '{}'", ident),
                ));
            }
        } else if content.peek(LitStr) {
            let lit: LitStr = content.parse()?;
            return Err(Error::new(
                lit.span(),
                "Raw string commands are not allowed. Use cmd!(\"...\") for shell validation.\n\
                 Change: command(\"...\") to command(cmd!(\"...\"))"
            ));
        } else {
            return Err(Error::new(
                content.span(),
                "expected cmd!(\"...\")",
            ));
        };
        
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
                    if content.peek(Ident) {
                        let ident: Ident = content.parse()?;
                        if ident == "cmd" {
                            content.parse::<Token![!]>()?;
                            let cmd_content;
                            syn::parenthesized!(cmd_content in content);
                            let lit: LitStr = cmd_content.parse().map_err(|_| {
                                Error::new(
                                    cmd_content.span(),
                                    "cmd! requires a string literal, e.g., cmd!(\"npm install\")"
                                )
                            })?;
                            let cmd_expr = CmdExpr::from_lit_str(&lit)?;
                            step.command = Some(CommandValue::new(cmd_expr));
                        } else {
                            return Err(Error::new(
                                ident.span(),
                                format!("expected cmd!(\"...\"), got '{}'", ident),
                            ));
                        }
                    } else if content.peek(LitStr) {
                        let lit: LitStr = content.parse()?;
                        return Err(Error::new(
                            lit.span(),
                            "Raw string commands are not allowed. Use cmd!(\"...\") for shell validation.\n\
                             Change: command: \"...\" to command: cmd!(\"...\")"
                        ));
                    } else {
                        return Err(Error::new(
                            content.span(),
                            "expected cmd!(\"...\")"
                        ));
                    }
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

    /// Generate tokens for this step with default plugins merged in
    fn to_tokens_with_default_plugins(&self, default_plugins: &[NestedValue]) -> TokenStream2 {
        match self {
            StepDef::Command(c) => c.to_tokens_with_default_plugins(default_plugins),
            StepDef::Wait(w) => w.to_tokens_inner(),
            StepDef::Block(b) => b.to_tokens_inner(),
            StepDef::Input(i) => i.to_tokens_inner(),
            StepDef::Trigger(t) => t.to_tokens_inner(),
            StepDef::Group(g) => g.to_tokens_with_default_plugins(default_plugins),
        }
    }

    /// Generate tokens for this step as a GroupStepsItem with default plugins
    fn to_group_step_tokens_with_default_plugins(
        &self,
        default_plugins: &[NestedValue],
    ) -> TokenStream2 {
        match self {
            StepDef::Command(c) => c.to_group_step_tokens_with_default_plugins(default_plugins),
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
                quote! { .continue_on_failure(true) }
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
                quote! { .continue_on_failure(true) }
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
/// Represents a validated command via cmd!("...")
/// All commands must use cmd!() for bashrs validation - raw strings are rejected.
#[derive(Clone)]
struct CommandValue(CmdExpr);

impl CommandValue {
    fn new(cmd: CmdExpr) -> Self {
        Self(cmd)
    }
    
    /// Get the command string value
    fn get_command_string(&self) -> String {
        self.0.command.clone()
    }
    
    /// Get the command name for allowlist validation
    fn get_command_name(&self) -> String {
        self.0.command_name.clone()
    }
    
    /// Get undefined vars flagged by bashrs (SC2154)
    /// These are vars that aren't defined inline in the script
    fn get_undefined_vars(&self) -> &[String] {
        &self.0.undefined_vars
    }
    
    /// Get span for error reporting
    fn span(&self) -> proc_macro2::Span {
        self.0.span
    }
}

struct CommandStepDef {
    command: Option<CommandValue>,
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
                    quote! { .required(#r) }
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
                    quote! { .required(#r) }
                } else {
                    quote! {}
                };
                let default_tokens = if let Some(d) = &f.default {
                    quote! { .default(::rust_buildkite::SelectFieldDefault::String(#d.to_string())) }
                } else {
                    quote! {}
                };
                let multiple_tokens = if let Some(m) = f.multiple {
                    quote! { .multiple(#m) }
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
    fn new_with_cmd(cmd_expr: CmdExpr) -> Self {
        Self {
            command: Some(CommandValue::new(cmd_expr)),
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

    /// Get the command name for validation (first word of the command)
    fn get_command_name(&self) -> Option<(String, proc_macro2::Span)> {
        self.command.as_ref().map(|cv| (cv.get_command_name(), cv.span()))
    }

    fn to_tokens_inner(&self) -> TokenStream2 {
        let cmd_value = self.command.as_ref().expect("command must be set");
        let cmd_string = cmd_value.get_command_string();
        let cmd_tokens = quote! { #cmd_string.to_string() };

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
            quote! { .soft_fail(Some(::rust_buildkite::SoftFail::Boolean(true))) }
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
                        __step_env.insert(#k.to_string(), ::rust_buildkite::serde_json::Value::String(#v.to_string()));
                    }
                })
                .collect();

            quote! {
                .env({
                    let mut __step_env = ::rust_buildkite::serde_json::Map::new();
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
                        __step_agents.insert(#k.to_string(), ::rust_buildkite::serde_json::Value::String(#v.to_string()));
                    }
                })
                .collect();

            quote! {
                .agents({
                    let mut __step_agents = ::rust_buildkite::serde_json::Map::new();
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
                    let mut __retry_obj = ::rust_buildkite::serde_json::Map::new();
                    #automatic_tokens
                    #manual_tokens
                    let __retry_value = ::rust_buildkite::serde_json::Value::Object(__retry_obj);
                    Some(::rust_buildkite::serde_json::from_value(__retry_value).expect("invalid retry config"))
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
                            ::rust_buildkite::serde_json::from_value(v).expect("invalid plugin")
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
                        ::rust_buildkite::serde_json::from_value(v).expect("invalid notify")
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
                    Some(::rust_buildkite::serde_json::from_value(__matrix_value).expect("invalid matrix"))
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
            quote! { .allow_dependency_failure(Some(::rust_buildkite::AllowDependencyFailure(true))) }
        } else {
            quote! {}
        };

        quote! {
            ::rust_buildkite::PipelineStepsItem::CommandStep(
                ::rust_buildkite::CommandStep::builder()
                    .command(Some(::rust_buildkite::CommandStepCommand::String(#cmd_tokens)))
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

    fn to_tokens_with_default_plugins(&self, default_plugins: &[NestedValue]) -> TokenStream2 {
        if default_plugins.is_empty() {
            return self.to_tokens_inner();
        }

        let all_plugins: Vec<&NestedValue> = default_plugins
            .iter()
            .chain(self.plugins.iter())
            .collect();

        let cmd_value = self.command.as_ref().expect("command must be set");

        let cmd_tokens = match &cmd_value.0 {
            #[cfg(feature = "bazel")]
            CommandSource::RuntimeBazel {
                base_cmd,
                flags_expr,
                target,
            } => {
                quote! {
                    format!("bazel {} {} {}", #base_cmd, #flags_expr, #target)
                }
            }
            _ => {
                let cmd_string = cmd_value.get_command_string();
                quote! { #cmd_string.to_string() }
            }
        };

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
            quote! { .soft_fail(Some(::rust_buildkite::SoftFail::Boolean(true))) }
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
                        __step_env.insert(#k.to_string(), ::rust_buildkite::serde_json::Value::String(#v.to_string()));
                    }
                })
                .collect();

            quote! {
                .env({
                    let mut __step_env = ::rust_buildkite::serde_json::Map::new();
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
                        __step_agents.insert(#k.to_string(), ::rust_buildkite::serde_json::Value::String(#v.to_string()));
                    }
                })
                .collect();

            quote! {
                .agents({
                    let mut __step_agents = ::rust_buildkite::serde_json::Map::new();
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
                    let mut __retry_obj = ::rust_buildkite::serde_json::Map::new();
                    #automatic_tokens
                    #manual_tokens
                    let __retry_value = ::rust_buildkite::serde_json::Value::Object(__retry_obj);
                    Some(::rust_buildkite::serde_json::from_value(__retry_value).expect("invalid retry config"))
                })
            }
        } else {
            quote! {}
        };

        let plugins_tokens = {
            let plugin_values: Vec<TokenStream2> =
                all_plugins.iter().map(|p| p.to_json_tokens()).collect();
            quote! {
                .plugins({
                    let __plugins_array = vec![#(#plugin_values),*];
                    Some(::rust_buildkite::Plugins::List(
                        ::rust_buildkite::PluginsList(__plugins_array.into_iter().map(|v| {
                            ::rust_buildkite::serde_json::from_value(v).expect("invalid plugin")
                        }).collect())
                    ))
                })
            }
        };

        let notify_tokens = if !self.notify.is_empty() {
            let notify_values: Vec<TokenStream2> =
                self.notify.iter().map(|n| n.to_json_tokens()).collect();
            quote! {
                .notify({
                    let __notify_array = vec![#(#notify_values),*];
                    Some(::rust_buildkite::CommandStepNotify(__notify_array.into_iter().map(|v| {
                        ::rust_buildkite::serde_json::from_value(v).expect("invalid notify")
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
                    Some(::rust_buildkite::serde_json::from_value(__matrix_value).expect("invalid matrix"))
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
            quote! { .allow_dependency_failure(Some(::rust_buildkite::AllowDependencyFailure(true))) }
        } else {
            quote! {}
        };

        quote! {
            ::rust_buildkite::PipelineStepsItem::CommandStep(
                ::rust_buildkite::CommandStep::builder()
                    .command(Some(::rust_buildkite::CommandStepCommand::String(#cmd_tokens)))
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
        let cmd_value = self.command.as_ref().expect("command must be set");
        let cmd_string = cmd_value.get_command_string();
        let cmd_tokens = quote! { #cmd_string.to_string() };

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
            quote! { .soft_fail(Some(::rust_buildkite::SoftFail::Boolean(true))) }
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
                quote! { __env_map.insert(#k.to_string(), ::rust_buildkite::serde_json::Value::String(#v.to_string())); }
            }).collect();
            quote! {
                .env({
                    let mut __env_map = ::rust_buildkite::serde_json::Map::new();
                    #(#inserts)*
                    Some(::rust_buildkite::Env(__env_map))
                })
            }
        } else {
            quote! {}
        };

        let agents_tokens = if !self.agents.is_empty() {
            let inserts: Vec<TokenStream2> = self.agents.iter().map(|(k, v)| {
                quote! { __agents_map.insert(#k.to_string(), ::rust_buildkite::serde_json::Value::String(#v.to_string())); }
            }).collect();
            quote! {
                .agents({
                    let mut __agents_map = ::rust_buildkite::serde_json::Map::new();
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
                        Some(::rust_buildkite::serde_json::from_value(__auto_value).expect("invalid automatic retry config"))
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
                        Some(::rust_buildkite::serde_json::from_value(__manual_value).expect("invalid manual retry config"))
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
                        ::rust_buildkite::serde_json::from_value(v).expect("invalid plugin")
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
                        ::rust_buildkite::serde_json::from_value(v).expect("invalid notify")
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
                    Some(::rust_buildkite::serde_json::from_value(__matrix_value).expect("invalid matrix"))
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
            quote! { .allow_dependency_failure(Some(::rust_buildkite::AllowDependencyFailure(true))) }
        } else {
            quote! {}
        };

        quote! {
            ::rust_buildkite::GroupStepsItem::CommandStep(
                ::rust_buildkite::CommandStep::builder()
                    .command(Some(::rust_buildkite::CommandStepCommand::String(#cmd_tokens)))
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

    /// Generate tokens for GroupStepsItem::CommandStep with default plugins merged
    fn to_group_step_tokens_with_default_plugins(
        &self,
        default_plugins: &[NestedValue],
    ) -> TokenStream2 {
        if default_plugins.is_empty() {
            return self.to_group_step_tokens();
        }

        let all_plugins: Vec<&NestedValue> = default_plugins
            .iter()
            .chain(self.plugins.iter())
            .collect();

        let cmd_value = self.command.as_ref().expect("command must be set");
        let cmd_string = cmd_value.get_command_string();
        let cmd_tokens = quote! { #cmd_string.to_string() };

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
            quote! { .soft_fail(Some(::rust_buildkite::SoftFail::Boolean(true))) }
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
            let inserts: Vec<TokenStream2> = self
                .env
                .iter()
                .map(|(k, v)| {
                    quote! { __env_map.insert(#k.to_string(), ::rust_buildkite::serde_json::Value::String(#v.to_string())); }
                })
                .collect();
            quote! {
                .env({
                    let mut __env_map = ::rust_buildkite::serde_json::Map::new();
                    #(#inserts)*
                    Some(::rust_buildkite::Env(__env_map))
                })
            }
        } else {
            quote! {}
        };

        let agents_tokens = if !self.agents.is_empty() {
            let inserts: Vec<TokenStream2> = self
                .agents
                .iter()
                .map(|(k, v)| {
                    quote! { __agents_map.insert(#k.to_string(), ::rust_buildkite::serde_json::Value::String(#v.to_string())); }
                })
                .collect();
            quote! {
                .agents({
                    let mut __agents_map = ::rust_buildkite::serde_json::Map::new();
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
                        Some(::rust_buildkite::serde_json::from_value(__auto_value).expect("invalid automatic retry config"))
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
                        Some(::rust_buildkite::serde_json::from_value(__manual_value).expect("invalid manual retry config"))
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

        let plugins_tokens = {
            let plugin_values: Vec<TokenStream2> =
                all_plugins.iter().map(|p| p.to_json_tokens()).collect();
            quote! {
                .plugins({
                    let __plugins_array = vec![#(#plugin_values),*];
                    Some(::rust_buildkite::Plugins::List(::rust_buildkite::PluginsList(__plugins_array.into_iter().map(|v| {
                        ::rust_buildkite::serde_json::from_value(v).expect("invalid plugin")
                    }).collect())))
                })
            }
        };

        let notify_tokens = if !self.notify.is_empty() {
            let notify_values: Vec<TokenStream2> =
                self.notify.iter().map(|n| n.to_json_tokens()).collect();
            quote! {
                .notify({
                    let __notify_array = vec![#(#notify_values),*];
                    Some(::rust_buildkite::CommandStepNotify(__notify_array.into_iter().map(|v| {
                        ::rust_buildkite::serde_json::from_value(v).expect("invalid notify")
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
                    Some(::rust_buildkite::serde_json::from_value(__matrix_value).expect("invalid matrix"))
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
            quote! { .allow_dependency_failure(Some(::rust_buildkite::AllowDependencyFailure(true))) }
        } else {
            quote! {}
        };

        quote! {
            ::rust_buildkite::GroupStepsItem::CommandStep(
                ::rust_buildkite::CommandStep::builder()
                    .command(Some(::rust_buildkite::CommandStepCommand::String(#cmd_tokens)))
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
            quote! { .allow_dependency_failure(Some(::rust_buildkite::AllowDependencyFailure(true))) }
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
            quote! { .allow_dependency_failure(Some(::rust_buildkite::AllowDependencyFailure(true))) }
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
            quote! { .allow_dependency_failure(Some(::rust_buildkite::AllowDependencyFailure(true))) }
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
            quote! { .allow_dependency_failure(Some(::rust_buildkite::AllowDependencyFailure(true))) }
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
            quote! { .async_(true) }
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
                    quote! { __build_env.insert(#k.to_string(), ::rust_buildkite::serde_json::Value::String(#v.to_string())); }
                }).collect();
                quote! {
                    .env({
                        let mut __build_env = ::rust_buildkite::serde_json::Map::new();
                        #(#env_inserts)*
                        Some(::rust_buildkite::Env(__build_env))
                    })
                }
            } else {
                quote! {}
            };
            let meta_data_tokens = if !build.meta_data.is_empty() {
                let md_inserts: Vec<TokenStream2> = build.meta_data.iter().map(|(k, v)| {
                    quote! { __build_meta.insert(#k.to_string(), ::rust_buildkite::serde_json::Value::String(#v.to_string())); }
                }).collect();
                quote! {
                    .meta_data({
                        let mut __build_meta = ::rust_buildkite::serde_json::Map::new();
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
            quote! { .soft_fail(Some(::rust_buildkite::SoftFail::Boolean(true))) }
        } else {
            quote! {}
        };

        let allow_dependency_failure_tokens = if self.allow_dependency_failure {
            quote! { .allow_dependency_failure(Some(::rust_buildkite::AllowDependencyFailure(true))) }
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
            quote! { .async_(Some(true)) }
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
                    quote! { __build_env.insert(#k.to_string(), ::rust_buildkite::serde_json::Value::String(#v.to_string())); }
                }).collect();
                quote! {
                    .env({
                        let mut __build_env = ::rust_buildkite::serde_json::Map::new();
                        #(#env_inserts)*
                        Some(::rust_buildkite::Env(__build_env))
                    })
                }
            } else {
                quote! {}
            };
            let meta_data_tokens = if !build.meta_data.is_empty() {
                let md_inserts: Vec<TokenStream2> = build.meta_data.iter().map(|(k, v)| {
                    quote! { __build_meta.insert(#k.to_string(), ::rust_buildkite::serde_json::Value::String(#v.to_string())); }
                }).collect();
                quote! {
                    .meta_data({
                        let mut __build_meta = ::rust_buildkite::serde_json::Map::new();
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
            quote! { .soft_fail(Some(::rust_buildkite::SoftFail::Boolean(true))) }
        } else {
            quote! {}
        };

        let allow_dependency_failure_tokens = if self.allow_dependency_failure {
            quote! { .allow_dependency_failure(Some(::rust_buildkite::AllowDependencyFailure(true))) }
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
                        ::rust_buildkite::serde_json::from_value(v).expect("invalid notify")
                    }).collect()))
                })
            }
        } else {
            quote! {}
        };

        let allow_dependency_failure_tokens = if self.allow_dependency_failure {
            quote! { .allow_dependency_failure(Some(::rust_buildkite::AllowDependencyFailure(true))) }
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

    /// Generate tokens with default plugins applied to nested steps
    fn to_tokens_with_default_plugins(&self, default_plugins: &[NestedValue]) -> TokenStream2 {
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

        let nested_steps: Vec<TokenStream2> = self
            .steps
            .iter()
            .map(|s| s.to_group_step_tokens_with_default_plugins(default_plugins))
            .collect();
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
            let notify_values: Vec<TokenStream2> =
                self.notify.iter().map(|n| n.to_json_tokens()).collect();
            quote! {
                .notify({
                    let __notify_array = vec![#(#notify_values),*];
                    Some(::rust_buildkite::BuildNotify(__notify_array.into_iter().map(|v| {
                        ::rust_buildkite::serde_json::from_value(v).expect("invalid notify")
                    }).collect()))
                })
            }
        } else {
            quote! {}
        };

        let allow_dependency_failure_tokens = if self.allow_dependency_failure {
            quote! { .allow_dependency_failure(Some(::rust_buildkite::AllowDependencyFailure(true))) }
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

/// Represents a parsed command from a string literal.
/// Uses bashrs for proper shell parsing and validation.
#[derive(Clone)]
struct CmdExpr {
    /// The command string
    command: String,
    /// The first command name (for allowlist validation)
    command_name: String,
    /// Variables that bashrs flagged as undefined (SC2154)
    /// These need to be validated against pipeline env/runtime_env
    undefined_vars: Vec<String>,
    /// Span for error reporting
    span: proc_macro2::Span,
}

impl CmdExpr {
    /// Parse a command from a string literal and validate with bashrs.
    /// Path existence is validated separately at pipeline level with allow_missing_paths context.
    fn from_lit_str(lit: &LitStr) -> Result<Self> {
        let command = lit.value();
        let span = lit.span();

        let undefined_vars = match Self::validate_with_bashrs(&command) {
            Ok(vars) => vars,
            Err(e) => return Err(Error::new(span, e)),
        };
        
        let command_name = Self::extract_command_name(&command);
        
        Ok(CmdExpr {
            command,
            command_name,
            undefined_vars,
            span,
        })
    }
    
    /// Validate the command string using bashrs linter.
    /// Returns Ok with list of undefined vars (SC2154), or Err for other issues.
    /// Undefined vars are passed to pipeline-level validation against env/runtime_env.
    fn validate_with_bashrs(command: &str) -> std::result::Result<Vec<String>, String> {
        use bashrs::linter::{lint_shell, Severity};
        
        let script = format!("#!/bin/bash\n{}", command);
        let result = lint_shell(&script);
        let undefined_vars: Vec<String> = result.diagnostics
            .iter()
            .filter(|d| d.code == "SC2154")
            .filter_map(|d| {
                let msg = &d.message;
                if let Some(start) = msg.find('\'') {
                    if let Some(end) = msg[start+1..].find('\'') {
                        return Some(msg[start+1..start+1+end].to_string());
                    }
                }
                None
            })
            .collect();

        let issues: Vec<_> = result.diagnostics
            .iter()
            .filter(|d| {
                (d.severity == Severity::Error || d.severity == Severity::Warning)
                && d.code != "SC2154"
            })
            .collect();
        
        if !issues.is_empty() {
            let error_msgs: Vec<String> = issues
                .iter()
                .map(|d| format!("  [{}] {}", d.code, d.message))
                .collect();
            return Err(format!(
                "Shell lint issues:\n{}",
                error_msgs.join("\n")
            ));
        }
        
        Ok(undefined_vars)
    }
    
    /// Extract the command name (first word) from a shell command.
    fn extract_command_name(command: &str) -> String {
        command
            .trim()
            .split_whitespace()
            .next()
            .unwrap_or("")
            .to_string()
    }
    
    /// Check if the command exists on the filesystem (for path-based commands).
    /// Returns Ok(()) if valid, Err with message if path doesn't exist.
    fn validate_path_exists(command_name: &str, allow_missing: &[&str]) -> std::result::Result<(), String> {
        use std::path::Path;
        if allow_missing.iter().any(|allowed| *allowed == command_name) {
            return Ok(());
        }
        if command_name.starts_with('/') || command_name.starts_with("./") {
            let path = Path::new(command_name);
            if !path.exists() {
                return Err(format!(
                    "Command path '{}' does not exist on the build machine.\n\
                     If this path will exist at runtime, add it to allow_missing_paths.",
                    command_name
                ));
            }
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Ok(metadata) = path.metadata() {
                    let mode = metadata.permissions().mode();
                    if mode & 0o111 == 0 {
                        return Err(format!(
                            "Command path '{}' exists but is not executable (mode: {:o}).",
                            command_name, mode
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    /// Generate code that produces the command string.
    fn to_tokens(&self) -> TokenStream2 {
        let cmd = &self.command;
        quote! { #cmd.to_string() }
    }
}

/// A macro for defining shell commands with bashrs validation.
///
/// This macro accepts a **string literal** containing a shell command.
/// At compile time, bashrs parses and lints the command for errors.
///
/// # Example
///
/// ```ignore
/// use rust_buildkite::cmd;
///
/// // Simple command
/// let c = cmd!("npm install --save-dev");
///
/// // Complex command with operators
/// let c = cmd!("npm install && npm test");
///
/// // With Rust format interpolation
/// let env = "production";
/// let c = cmd!(&format!("./deploy.sh {}", env));
/// ```
#[proc_macro]
pub fn cmd(input: TokenStream) -> TokenStream {
    let lit = match syn::parse::<LitStr>(input) {
        Ok(lit) => lit,
        Err(_) => {
            return syn::Error::new(
                proc_macro2::Span::call_site(),
                "cmd! requires a string literal, e.g., cmd!(\"npm install\")"
            )
            .to_compile_error()
            .into();
        }
    };

    match CmdExpr::from_lit_str(&lit) {
        Ok(cmd_expr) => cmd_expr.to_tokens().into(),
        Err(err) => err.to_compile_error().into(),
    }
}
