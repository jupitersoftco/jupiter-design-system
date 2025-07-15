use jupiter_design_system::prelude::*;

fn main() {
    println!("🌈 Psychedelic Theme Demo 🌈");
    println!("===========================");

    // Create the psychedelic color provider
    let psychedelic_colors = PsychedelicColors::default();
    let theme = PsychedelicTheme::new();

    println!("🎨 Theme: {}", theme.name());
    println!();

    // Showcase vibrant buttons
    println!("🔘 Psychedelic Buttons:");

    let primary_button = button_styles(psychedelic_colors.clone())
        .primary()
        .large()
        .classes();
    println!("  Primary: {}", primary_button);

    let secondary_button = button_styles(psychedelic_colors.clone())
        .secondary()
        .medium()
        .classes();
    println!("  Secondary: {}", secondary_button);

    let accent_button = button_styles(psychedelic_colors.clone())
        .variant_str("accent")
        .small()
        .classes();
    println!("  Accent: {}", accent_button);
    println!();

    // Showcase colorful text
    println!("📝 Psychedelic Text:");

    let title_text = text_styles(psychedelic_colors.clone())
        .title()
        .primary()
        .center()
        .classes();
    println!("  Title: {}", title_text);

    let accent_text = text_styles(psychedelic_colors.clone())
        .heading()
        .accent()
        .classes();
    println!("  Accent Heading: {}", accent_text);

    let secondary_text = text_styles(psychedelic_colors.clone())
        .body()
        .secondary()
        .classes();
    println!("  Secondary Body: {}", secondary_text);
    println!();

    // Showcase vibrant cards
    println!("🎴 Psychedelic Cards:");

    let vibrant_card = CardStyles::new(psychedelic_colors.clone())
        .elevated_surface()
        .floating_elevation()
        .hoverable_interaction()
        .classes();
    println!("  Vibrant Card: {}", vibrant_card);

    let branded_card = CardStyles::new(psychedelic_colors.clone())
        .branded_surface()
        .modal_elevation()
        .classes();
    println!("  Branded Card: {}", branded_card);
    println!();

    // Showcase selection components
    println!("✨ Psychedelic Selection:");

    let (container, item) = selection_classes_from_strings(
        psychedelic_colors.clone(),
        "multiple",   // behavior
        "selected",   // state
        "chip",       // display
        "horizontal", // layout
        "md",         // size
        "prominent",  // interaction
        true,         // show_counts
    );
    println!("  Container: {}", container);
    println!("  Item: {}", item);
    println!();

    // Color palette showcase
    println!("🎨 Psychedelic Color Palette:");
    let palette = psychedelic_colors.palette();
    println!("  Primary: {}", palette.primary);
    println!("  Secondary: {}", palette.secondary);
    println!("  Accent: {}", palette.accent);
    println!("  Success: {}", palette.success);
    println!("  Warning: {}", palette.warning);
    println!("  Error: {}", palette.error);
    println!("  Info: {}", palette.info);
    println!();

    println!("🚀 Ready to get psychedelic with your UI! 🚀");
}
