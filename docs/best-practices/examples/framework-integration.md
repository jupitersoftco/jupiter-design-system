# Framework Integration Examples

## 🎯 Overview

This guide provides comprehensive examples for integrating the Jupiter Design System with popular Rust web frameworks, demonstrating best practices for each ecosystem.

## 🔥 Dioxus Integration

### Basic Setup

```rust
// Cargo.toml
[dependencies]
dioxus = "0.4"
dioxus-web = "0.4"
jupiter-design-system = { path = "../jupiter-design-system" }
```

### Theme Provider Context

```rust
use dioxus::prelude::*;
use jupiter_design_system::prelude::*;

#[derive(Clone, Debug)]
pub struct ThemeContext {
    pub colors: VibeColors,
    pub dark_mode: bool,
}

impl Default for ThemeContext {
    fn default() -> Self {
        Self {
            colors: VibeColors::new(),
            dark_mode: false,
        }
    }
}

#[component]
pub fn ThemeProvider(children: Element) -> Element {
    let mut theme_context = use_signal(ThemeContext::default);
    
    // Theme switching logic
    let toggle_theme = move |_| {
        let mut theme = theme_context.write();
        theme.dark_mode = !theme.dark_mode;
        
        // Update colors based on theme
        theme.colors = if theme.dark_mode {
            // Use dark theme colors
            VibeColors::with_overrides(|palette| {
                palette.background = "gray-900".to_string();
                palette.text_primary = "gray-100".to_string();
                palette.surface = "gray-800".to_string();
            })
        } else {
            VibeColors::new()
        };
    };
    
    rsx! {
        div {
            data_theme: if theme_context.read().dark_mode { "dark" } else { "light" },
            class: "min-h-screen transition-colors duration-200",
            style: "background-color: {theme_context.read().colors.resolve_color(Color::Background)}",
            
            // Theme toggle button
            button {
                class: "{button_styles(theme_context.read().colors.clone()).ghost().classes()}",
                onclick: toggle_theme,
                "🌙 Toggle Theme"
            }
            
            // Provide theme context to children
            use_context_provider(|| theme_context.read().clone()),
            {children}
        }
    }
}
```

### Component Examples

```rust
#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    pub variant: ButtonVariant,
    pub size: Size,
    pub full_width: Option<bool>,
    pub disabled: Option<bool>,
    pub loading: Option<bool>,
    pub onclick: Option<EventHandler<MouseEvent>>,
    pub children: Element,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let theme = use_context::<ThemeContext>();
    
    let state = if props.disabled.unwrap_or(false) {
        ButtonState::Disabled
    } else if props.loading.unwrap_or(false) {
        ButtonState::Loading
    } else {
        ButtonState::Default
    };
    
    let classes = button_styles(theme.colors.clone())
        .variant(props.variant)
        .size(props.size)
        .state(state)
        .full_width(props.full_width.unwrap_or(false))
        .classes();
    
    let onclick = props.onclick.unwrap_or_else(|| EventHandler::new(|_| {}));
    
    rsx! {
        button {
            class: "{classes}",
            disabled: props.disabled.unwrap_or(false),
            onclick: move |e| {
                if !props.disabled.unwrap_or(false) && !props.loading.unwrap_or(false) {
                    onclick.call(e);
                }
            },
            
            if props.loading.unwrap_or(false) {
                span {
                    class: "inline-block animate-spin rounded-full h-4 w-4 border-b-2 border-current mr-2",
                    "aria-hidden": "true"
                }
            }
            
            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CardProps {
    pub elevation: Option<CardElevation>,
    pub spacing: Option<CardSpacing>,
    pub interactive: Option<bool>,
    pub children: Element,
}

#[component]
pub fn Card(props: CardProps) -> Element {
    let theme = use_context::<ThemeContext>();
    
    let classes = card_styles(theme.colors.clone())
        .elevation(props.elevation.unwrap_or(CardElevation::Low))
        .spacing(props.spacing.unwrap_or(CardSpacing::Medium))
        .interaction(if props.interactive.unwrap_or(false) {
            CardInteraction::Clickable
        } else {
            CardInteraction::None
        })
        .classes();
    
    rsx! {
        div {
            class: "{classes}",
            {props.children}
        }
    }
}
```

### Form Components

```rust
#[derive(Props, Clone, PartialEq)]
pub struct FormFieldProps {
    pub label: String,
    pub placeholder: Option<String>,
    pub required: Option<bool>,
    pub error: Option<String>,
    pub help_text: Option<String>,
    pub value: String,
    pub onchange: EventHandler<Event<FormData>>,
}

#[component]
pub fn FormField(props: FormFieldProps) -> Element {
    let theme = use_context::<ThemeContext>();
    let has_error = props.error.is_some();
    
    let field_id = use_memo(|| format!("field-{}", uuid::Uuid::new_v4()));
    let error_id = use_memo(|| format!("error-{}", uuid::Uuid::new_v4()));
    let help_id = use_memo(|| format!("help-{}", uuid::Uuid::new_v4()));
    
    let label_classes = text_styles(theme.colors.clone())
        .typography(Typography::Label)
        .color(Color::TextPrimary)
        .weight(FontWeight::Medium)
        .classes();
    
    let input_classes = if has_error {
        interactive_input(theme.colors.clone())
            .base_style()
            .border_color(Color::Error)
            .hover().border_color(Color::Error)
            .focus().border_color(Color::Error).ring_color(Color::Error)
            .build()
    } else {
        interactive_input(theme.colors.clone())
            .base_style()
            .hover().border_primary()
            .focus().border_primary().ring_primary()
            .build()
    };
    
    let error_classes = text_styles(theme.colors.clone())
        .typography(Typography::Caption)
        .color(Color::Error)
        .classes();
    
    let help_classes = text_styles(theme.colors.clone())
        .typography(Typography::Caption)
        .color(Color::TextTertiary)
        .classes();
    
    let container_classes = layout_styles(theme.colors.clone())
        .direction_vertical()
        .spacing_sm()
        .classes();
    
    rsx! {
        div {
            class: "{container_classes}",
            
            label {
                "for": "{field_id}",
                class: "{label_classes}",
                "{props.label}"
                if props.required.unwrap_or(false) {
                    span {
                        class: "ml-1 text-red-500",
                        "aria-label": "required",
                        "*"
                    }
                }
            }
            
            input {
                id: "{field_id}",
                class: "{input_classes}",
                "type": "text",
                placeholder: props.placeholder.unwrap_or_default(),
                value: props.value,
                required: props.required.unwrap_or(false),
                "aria-invalid": has_error.to_string(),
                "aria-describedby": format!("{} {}", 
                    props.help_text.as_ref().map(|_| help_id.clone()).unwrap_or_default(),
                    props.error.as_ref().map(|_| error_id.clone()).unwrap_or_default()
                ),
                oninput: move |e| props.onchange.call(e)
            }
            
            if let Some(help) = props.help_text {
                div {
                    id: "{help_id}",
                    class: "{help_classes}",
                    "{help}"
                }
            }
            
            if let Some(error) = props.error {
                div {
                    id: "{error_id}",
                    class: "{error_classes}",
                    "role": "alert",
                    "aria-live": "polite",
                    "{error}"
                }
            }
        }
    }
}
```

### Complete Application Example

```rust
#[component]
fn App() -> Element {
    let mut user_name = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut email_error = use_signal(Option::<String>::None);
    let mut is_loading = use_signal(false);
    
    let validate_email = move |email: &str| -> Option<String> {
        if email.is_empty() {
            Some("Email is required".to_string())
        } else if !email.contains('@') {
            Some("Please enter a valid email address".to_string())
        } else {
            None
        }
    };
    
    let handle_submit = move |e: Event<FormData>| {
        e.prevent_default();
        
        let email_validation = validate_email(&email.read());
        email_error.set(email_validation.clone());
        
        if email_validation.is_none() {
            is_loading.set(true);
            
            // Simulate API call
            spawn(async move {
                gloo_timers::future::TimeoutFuture::new(2000).await;
                is_loading.set(false);
                println!("User registered: {}", email.read());
            });
        }
    };
    
    rsx! {
        ThemeProvider {
            div {
                class: "min-h-screen flex items-center justify-center p-4",
                
                Card {
                    elevation: CardElevation::Medium,
                    spacing: CardSpacing::Large,
                    
                    div {
                        class: "w-full max-w-md",
                        
                        h1 {
                            class: "{text_styles(use_context::<ThemeContext>().colors.clone())
                                .typography(Typography::Heading1)
                                .color(Color::TextPrimary)
                                .classes()} text-center mb-6",
                            "Create Account"
                        }
                        
                        form {
                            class: "space-y-4",
                            onsubmit: handle_submit,
                            
                            FormField {
                                label: "Full Name".to_string(),
                                placeholder: Some("Enter your full name".to_string()),
                                required: Some(true),
                                value: user_name.read().clone(),
                                onchange: move |e| user_name.set(e.data.value().clone())
                            }
                            
                            FormField {
                                label: "Email Address".to_string(),
                                placeholder: Some("Enter your email".to_string()),
                                required: Some(true),
                                value: email.read().clone(),
                                error: email_error.read().clone(),
                                help_text: Some("We'll never share your email with anyone else.".to_string()),
                                onchange: move |e| {
                                    email.set(e.data.value().clone());
                                    email_error.set(None);
                                }
                            }
                            
                            FormField {
                                label: "Password".to_string(),
                                placeholder: Some("Create a strong password".to_string()),
                                required: Some(true),
                                value: password.read().clone(),
                                help_text: Some("Must be at least 8 characters long.".to_string()),
                                onchange: move |e| password.set(e.data.value().clone())
                            }
                            
                            div {
                                class: "pt-4",
                                
                                Button {
                                    variant: ButtonVariant::Primary,
                                    size: Size::Large,
                                    full_width: Some(true),
                                    loading: Some(is_loading.read()),
                                    disabled: Some(is_loading.read()),
                                    onclick: |_| {}, // Form submission handled by form
                                    "Create Account"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    dioxus_web::launch(App);
}
```

## 🌐 Yew Integration

### Basic Setup

```rust
// Cargo.toml
[dependencies]
yew = "0.21"
jupiter-design-system = { path = "../jupiter-design-system" }
wasm-bindgen = "0.2"
```

### Theme Context Provider

```rust
use yew::prelude::*;
use jupiter_design_system::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ThemeContext {
    pub colors: VibeColors,
    pub dark_mode: bool,
}

impl Default for ThemeContext {
    fn default() -> Self {
        Self {
            colors: VibeColors::new(),
            dark_mode: false,
        }
    }
}

pub enum ThemeAction {
    ToggleDarkMode,
    UpdateColors(VibeColors),
}

impl Reducible for ThemeContext {
    type Action = ThemeAction;
    
    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            ThemeAction::ToggleDarkMode => {
                let dark_mode = !self.dark_mode;
                let colors = if dark_mode {
                    VibeColors::with_overrides(|palette| {
                        palette.background = "gray-900".to_string();
                        palette.text_primary = "gray-100".to_string();
                        palette.surface = "gray-800".to_string();
                    })
                } else {
                    VibeColors::new()
                };
                
                Self { colors, dark_mode }.into()
            }
            ThemeAction::UpdateColors(colors) => {
                Self { colors, ..(*self).clone() }.into()
            }
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ThemeProviderProps {
    pub children: Children,
}

#[function_component(ThemeProvider)]
pub fn theme_provider(props: &ThemeProviderProps) -> Html {
    let theme_context = use_reducer(ThemeContext::default);
    
    html! {
        <ContextProvider<UseReducerHandle<ThemeContext>> context={theme_context}>
            <div class="min-h-screen transition-colors duration-200">
                {for props.children.iter()}
            </div>
        </ContextProvider<UseReducerHandle<ThemeContext>>>
    }
}
```

### Component Examples

```rust
#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub variant: ButtonVariant,
    pub size: Size,
    #[prop_or_default]
    pub full_width: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub loading: bool,
    pub onclick: Callback<MouseEvent>,
    pub children: Children,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let theme = use_context::<UseReducerHandle<ThemeContext>>()
        .expect("Theme context not found");
    
    let state = if props.disabled {
        ButtonState::Disabled
    } else if props.loading {
        ButtonState::Loading
    } else {
        ButtonState::Default
    };
    
    let classes = button_styles(theme.colors.clone())
        .variant(props.variant)
        .size(props.size)
        .state(state)
        .full_width(props.full_width)
        .classes();
    
    let onclick = props.onclick.clone();
    
    html! {
        <button
            class={classes}
            disabled={props.disabled}
            onclick={move |e| {
                if !props.disabled && !props.loading {
                    onclick.emit(e);
                }
            }}
        >
            if props.loading {
                <span class="inline-block animate-spin rounded-full h-4 w-4 border-b-2 border-current mr-2" aria-hidden="true"></span>
            }
            {for props.children.iter()}
        </button>
    }
}

#[derive(Properties, PartialEq)]
pub struct FormFieldProps {
    pub label: String,
    pub placeholder: Option<String>,
    #[prop_or_default]
    pub required: bool,
    pub error: Option<String>,
    pub help_text: Option<String>,
    pub value: String,
    pub onchange: Callback<String>,
}

#[function_component(FormField)]
pub fn form_field(props: &FormFieldProps) -> Html {
    let theme = use_context::<UseReducerHandle<ThemeContext>>()
        .expect("Theme context not found");
    
    let has_error = props.error.is_some();
    let field_id = use_memo(|_| format!("field-{}", uuid::Uuid::new_v4()), ());
    let error_id = use_memo(|_| format!("error-{}", uuid::Uuid::new_v4()), ());
    let help_id = use_memo(|_| format!("help-{}", uuid::Uuid::new_v4()), ());
    
    let label_classes = text_styles(theme.colors.clone())
        .typography(Typography::Label)
        .color(Color::TextPrimary)
        .weight(FontWeight::Medium)
        .classes();
    
    let input_classes = if has_error {
        interactive_input(theme.colors.clone())
            .base_style()
            .border_color(Color::Error)
            .hover().border_color(Color::Error)
            .focus().border_color(Color::Error).ring_color(Color::Error)
            .build()
    } else {
        interactive_input(theme.colors.clone())
            .base_style()
            .hover().border_primary()
            .focus().border_primary().ring_primary()
            .build()
    };
    
    let error_classes = text_styles(theme.colors.clone())
        .typography(Typography::Caption)
        .color(Color::Error)
        .classes();
    
    let help_classes = text_styles(theme.colors.clone())
        .typography(Typography::Caption)
        .color(Color::TextTertiary)
        .classes();
    
    let container_classes = layout_styles(theme.colors.clone())
        .direction_vertical()
        .spacing_sm()
        .classes();
    
    let onchange = props.onchange.clone();
    
    html! {
        <div class={container_classes}>
            <label for={field_id.clone()} class={label_classes}>
                {&props.label}
                if props.required {
                    <span class="ml-1 text-red-500" aria-label="required">{"*"}</span>
                }
            </label>
            
            <input
                id={field_id.clone()}
                class={input_classes}
                type="text"
                placeholder={props.placeholder.clone().unwrap_or_default()}
                value={props.value.clone()}
                required={props.required}
                aria-invalid={has_error.to_string()}
                aria-describedby={format!("{} {}", 
                    props.help_text.as_ref().map(|_| help_id.clone()).unwrap_or_default(),
                    props.error.as_ref().map(|_| error_id.clone()).unwrap_or_default()
                )}
                oninput={move |e| onchange.emit(e.target_unchecked_into::<web_sys::HtmlInputElement>().value())}
            />
            
            if let Some(help) = &props.help_text {
                <div id={help_id.clone()} class={help_classes}>
                    {help}
                </div>
            }
            
            if let Some(error) = &props.error {
                <div id={error_id.clone()} class={error_classes} role="alert" aria-live="polite">
                    {error}
                </div>
            }
        </div>
    }
}
```

### Complete Yew Application

```rust
#[derive(Default)]
struct AppState {
    user_name: String,
    email: String,
    password: String,
    email_error: Option<String>,
    is_loading: bool,
}

pub enum AppMessage {
    UpdateUserName(String),
    UpdateEmail(String),
    UpdatePassword(String),
    SubmitForm,
    FormSubmitted,
}

#[function_component(App)]
fn app() -> Html {
    let state = use_reducer(AppState::default);
    
    let validate_email = |email: &str| -> Option<String> {
        if email.is_empty() {
            Some("Email is required".to_string())
        } else if !email.contains('@') {
            Some("Please enter a valid email address".to_string())
        } else {
            None
        }
    };
    
    let handle_submit = {
        let state = state.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            let email_validation = validate_email(&state.email);
            
            if email_validation.is_none() {
                state.dispatch(AppMessage::SubmitForm);
                
                // Simulate API call
                let state = state.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    gloo_timers::future::sleep(std::time::Duration::from_secs(2)).await;
                    state.dispatch(AppMessage::FormSubmitted);
                });
            }
        })
    };
    
    html! {
        <ThemeProvider>
            <div class="min-h-screen flex items-center justify-center p-4">
                <div class="w-full max-w-md">
                    <h1 class="text-3xl font-bold text-center mb-6">{"Create Account"}</h1>
                    
                    <form class="space-y-4" onsubmit={handle_submit}>
                        <FormField
                            label="Full Name"
                            placeholder="Enter your full name"
                            required={true}
                            value={state.user_name.clone()}
                            onchange={let state = state.clone(); move |value| state.dispatch(AppMessage::UpdateUserName(value))}
                        />
                        
                        <FormField
                            label="Email Address"
                            placeholder="Enter your email"
                            required={true}
                            value={state.email.clone()}
                            error={state.email_error.clone()}
                            help_text="We'll never share your email with anyone else."
                            onchange={let state = state.clone(); move |value| state.dispatch(AppMessage::UpdateEmail(value))}
                        />
                        
                        <FormField
                            label="Password"
                            placeholder="Create a strong password"
                            required={true}
                            value={state.password.clone()}
                            help_text="Must be at least 8 characters long."
                            onchange={let state = state.clone(); move |value| state.dispatch(AppMessage::UpdatePassword(value))}
                        />
                        
                        <div class="pt-4">
                            <Button
                                variant={ButtonVariant::Primary}
                                size={Size::Large}
                                full_width={true}
                                loading={state.is_loading}
                                disabled={state.is_loading}
                                onclick={Callback::from(|_| {})}
                            >
                                {"Create Account"}
                            </Button>
                        </div>
                    </form>
                </div>
            </div>
        </ThemeProvider>
    }
}

impl Reducible for AppState {
    type Action = AppMessage;
    
    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            AppMessage::UpdateUserName(name) => {
                Self { user_name: name, ..(*self).clone() }.into()
            }
            AppMessage::UpdateEmail(email) => {
                Self { 
                    email,
                    email_error: None, // Clear error when user types
                    ..(*self).clone() 
                }.into()
            }
            AppMessage::UpdatePassword(password) => {
                Self { password, ..(*self).clone() }.into()
            }
            AppMessage::SubmitForm => {
                Self { is_loading: true, ..(*self).clone() }.into()
            }
            AppMessage::FormSubmitted => {
                Self { is_loading: false, ..(*self).clone() }.into()
            }
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
```

## ⚡ Leptos Integration

### Basic Setup

```rust
// Cargo.toml
[dependencies]
leptos = "0.5"
leptos_meta = "0.5"
leptos_router = "0.5"
jupiter-design-system = { path = "../jupiter-design-system" }
```

### Theme Context

```rust
use leptos::*;
use jupiter_design_system::prelude::*;

#[derive(Clone, Debug)]
pub struct ThemeContext {
    pub colors: VibeColors,
    pub dark_mode: bool,
}

impl Default for ThemeContext {
    fn default() -> Self {
        Self {
            colors: VibeColors::new(),
            dark_mode: false,
        }
    }
}

#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    let (theme, set_theme) = create_signal(ThemeContext::default());
    
    let toggle_theme = move |_| {
        set_theme.update(|theme| {
            theme.dark_mode = !theme.dark_mode;
            theme.colors = if theme.dark_mode {
                VibeColors::with_overrides(|palette| {
                    palette.background = "gray-900".to_string();
                    palette.text_primary = "gray-100".to_string();
                    palette.surface = "gray-800".to_string();
                })
            } else {
                VibeColors::new()
            };
        });
    };
    
    provide_context(theme);
    provide_context(set_theme);
    
    view! {
        <div class="min-h-screen transition-colors duration-200">
            <button
                class={move || button_styles(theme.get().colors.clone()).ghost().classes()}
                on:click=toggle_theme
            >
                "🌙 Toggle Theme"
            </button>
            {children()}
        </div>
    }
}
```

### Component Examples

```rust
#[component]
pub fn Button(
    variant: ButtonVariant,
    size: Size,
    #[prop(optional)] full_width: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] loading: bool,
    #[prop(optional)] on_click: Option<Box<dyn Fn() + 'static>>,
    children: Children,
) -> impl IntoView {
    let theme = use_context::<ReadSignal<ThemeContext>>()
        .expect("Theme context not found");
    
    let state = if disabled {
        ButtonState::Disabled
    } else if loading {
        ButtonState::Loading
    } else {
        ButtonState::Default
    };
    
    let classes = move || {
        button_styles(theme.get().colors.clone())
            .variant(variant)
            .size(size)
            .state(state)
            .full_width(full_width)
            .classes()
    };
    
    let handle_click = move |_| {
        if !disabled && !loading {
            if let Some(callback) = &on_click {
                callback();
            }
        }
    };
    
    view! {
        <button
            class=classes
            disabled=disabled
            on:click=handle_click
        >
            {move || loading.then(|| view! {
                <span class="inline-block animate-spin rounded-full h-4 w-4 border-b-2 border-current mr-2" aria-hidden="true"></span>
            })}
            {children()}
        </button>
    }
}

#[component]
pub fn FormField(
    label: String,
    #[prop(optional)] placeholder: String,
    #[prop(optional)] required: bool,
    #[prop(optional)] error: Option<String>,
    #[prop(optional)] help_text: Option<String>,
    value: ReadSignal<String>,
    set_value: WriteSignal<String>,
) -> impl IntoView {
    let theme = use_context::<ReadSignal<ThemeContext>>()
        .expect("Theme context not found");
    
    let has_error = error.is_some();
    let field_id = format!("field-{}", uuid::Uuid::new_v4());
    let error_id = format!("error-{}", uuid::Uuid::new_v4());
    let help_id = format!("help-{}", uuid::Uuid::new_v4());
    
    let label_classes = move || {
        text_styles(theme.get().colors.clone())
            .typography(Typography::Label)
            .color(Color::TextPrimary)
            .weight(FontWeight::Medium)
            .classes()
    };
    
    let input_classes = move || {
        if has_error {
            interactive_input(theme.get().colors.clone())
                .base_style()
                .border_color(Color::Error)
                .hover().border_color(Color::Error)
                .focus().border_color(Color::Error).ring_color(Color::Error)
                .build()
        } else {
            interactive_input(theme.get().colors.clone())
                .base_style()
                .hover().border_primary()
                .focus().border_primary().ring_primary()
                .build()
        }
    };
    
    let error_classes = move || {
        text_styles(theme.get().colors.clone())
            .typography(Typography::Caption)
            .color(Color::Error)
            .classes()
    };
    
    let help_classes = move || {
        text_styles(theme.get().colors.clone())
            .typography(Typography::Caption)
            .color(Color::TextTertiary)
            .classes()
    };
    
    let container_classes = move || {
        layout_styles(theme.get().colors.clone())
            .direction_vertical()
            .spacing_sm()
            .classes()
    };
    
    view! {
        <div class=container_classes>
            <label for=field_id.clone() class=label_classes>
                {label}
                {move || required.then(|| view! {
                    <span class="ml-1 text-red-500" aria-label="required">"*"</span>
                })}
            </label>
            
            <input
                id=field_id
                class=input_classes
                type="text"
                placeholder=placeholder
                prop:value=value
                required=required
                aria-invalid=has_error.to_string()
                aria-describedby=format!("{} {}", 
                    help_text.as_ref().map(|_| help_id.clone()).unwrap_or_default(),
                    error.as_ref().map(|_| error_id.clone()).unwrap_or_default()
                )
                on:input=move |e| set_value.set(event_target_value(&e))
            />
            
            {move || help_text.as_ref().map(|help| view! {
                <div id=help_id.clone() class=help_classes>
                    {help}
                </div>
            })}
            
            {move || error.as_ref().map(|err| view! {
                <div id=error_id.clone() class=error_classes role="alert" aria-live="polite">
                    {err}
                </div>
            })}
        </div>
    }
}
```

### Complete Leptos Application

```rust
#[component]
fn App() -> impl IntoView {
    let (user_name, set_user_name) = create_signal(String::new());
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (email_error, set_email_error) = create_signal(Option::<String>::None);
    let (is_loading, set_is_loading) = create_signal(false);
    
    let validate_email = |email: &str| -> Option<String> {
        if email.is_empty() {
            Some("Email is required".to_string())
        } else if !email.contains('@') {
            Some("Please enter a valid email address".to_string())
        } else {
            None
        }
    };
    
    let handle_submit = move |e: web_sys::Event| {
        e.prevent_default();
        
        let email_validation = validate_email(&email.get());
        set_email_error.set(email_validation.clone());
        
        if email_validation.is_none() {
            set_is_loading.set(true);
            
            // Simulate API call
            spawn_local(async move {
                gloo_timers::future::sleep(std::time::Duration::from_secs(2)).await;
                set_is_loading.set(false);
                logging::log!("User registered: {}", email.get());
            });
        }
    };
    
    view! {
        <ThemeProvider>
            <div class="min-h-screen flex items-center justify-center p-4">
                <div class="w-full max-w-md">
                    <h1 class="text-3xl font-bold text-center mb-6">"Create Account"</h1>
                    
                    <form class="space-y-4" on:submit=handle_submit>
                        <FormField
                            label="Full Name".to_string()
                            placeholder="Enter your full name".to_string()
                            required=true
                            value=user_name
                            set_value=set_user_name
                        />
                        
                        <FormField
                            label="Email Address".to_string()
                            placeholder="Enter your email".to_string()
                            required=true
                            value=email
                            set_value=set_email
                            error=email_error.get()
                            help_text=Some("We'll never share your email with anyone else.".to_string())
                        />
                        
                        <FormField
                            label="Password".to_string()
                            placeholder="Create a strong password".to_string()
                            required=true
                            value=password
                            set_value=set_password
                            help_text=Some("Must be at least 8 characters long.".to_string())
                        />
                        
                        <div class="pt-4">
                            <Button
                                variant=ButtonVariant::Primary
                                size=Size::Large
                                full_width=true
                                loading=is_loading.get()
                                disabled=is_loading.get()
                                on_click=Some(Box::new(|| {}))
                            >
                                "Create Account"
                            </Button>
                        </div>
                    </form>
                </div>
            </div>
        </ThemeProvider>
    }
}

fn main() {
    leptos::mount_to_body(App);
}
```

## 🔧 Integration Best Practices

### Performance Optimization

```rust
// Memoized component classes
#[component]
pub fn OptimizedButton(
    variant: ButtonVariant,
    size: Size,
    children: Children,
) -> impl IntoView {
    let theme = use_context::<ReadSignal<ThemeContext>>()
        .expect("Theme context not found");
    
    // Memoize classes to prevent unnecessary recalculation
    let classes = create_memo(move |_| {
        button_styles(theme.get().colors.clone())
            .variant(variant)
            .size(size)
            .classes()
    });
    
    view! {
        <button class=move || classes.get()>
            {children()}
        </button>
    }
}

// Cached theme provider
use std::collections::HashMap;
use std::sync::Arc;

lazy_static! {
    static ref THEME_CACHE: Arc<std::sync::RwLock<HashMap<String, String>>> = 
        Arc::new(std::sync::RwLock::new(HashMap::new()));
}

pub fn get_cached_classes(key: &str, generator: impl FnOnce() -> String) -> String {
    {
        let cache = THEME_CACHE.read().unwrap();
        if let Some(classes) = cache.get(key) {
            return classes.clone();
        }
    }
    
    let classes = generator();
    
    {
        let mut cache = THEME_CACHE.write().unwrap();
        cache.insert(key.to_string(), classes.clone());
    }
    
    classes
}
```

### Error Handling

```rust
// Error boundary component
#[component]
pub fn ErrorBoundary(children: Children) -> impl IntoView {
    let (error, set_error) = create_signal(None::<String>);
    
    // Provide error context
    provide_context(set_error);
    
    view! {
        {move || match error.get() {
            Some(err) => view! {
                <div class="min-h-screen flex items-center justify-center p-4">
                    <div class="text-center">
                        <h1 class="text-2xl font-bold text-red-600 mb-4">"Something went wrong"</h1>
                        <p class="text-gray-600 mb-4">{err}</p>
                        <Button
                            variant=ButtonVariant::Primary
                            size=Size::Medium
                            on_click=Some(Box::new(move || set_error.set(None)))
                        >
                            "Try Again"
                        </Button>
                    </div>
                </div>
            },
            None => children(),
        }}
    }
}
```

This comprehensive framework integration guide ensures developers can successfully implement the Jupiter Design System across all major Rust web frameworks with consistent patterns and best practices.