/// Product Builder - Chainable API for product component CSS generation
///
/// This builder provides a fluent interface for creating product component
/// classes based on Jupiter Design System patterns.
use crate::core::color::ColorProvider;
use crate::patterns::product::*;

/// Builder for product component CSS classes
#[derive(Debug, Clone)]
pub struct ProductBuilder<C: ColorProvider> {
    pattern: ProductCardPattern,
    colors: C,
    custom_classes: Vec<String>,
}

impl<C: ColorProvider> ProductBuilder<C> {
    /// Create a new product builder
    pub fn new(colors: C) -> Self {
        Self {
            pattern: ProductCardPattern::new(),
            colors,
            custom_classes: Vec::new(),
        }
    }

    /// Set display pattern to list item
    pub fn list_item(mut self) -> Self {
        self.pattern = self.pattern.display(ProductDisplayPattern::ListItem);
        self
    }

    /// Set display pattern to featured
    pub fn featured(mut self) -> Self {
        self.pattern = self.pattern.display(ProductDisplayPattern::Featured);
        self
    }

    /// Set display pattern to tile
    pub fn tile(mut self) -> Self {
        self.pattern = self.pattern.display(ProductDisplayPattern::Tile);
        self
    }

    /// Set display pattern to showcase
    pub fn showcase(mut self) -> Self {
        self.pattern = self.pattern.display(ProductDisplayPattern::Showcase);
        self
    }

    /// Set display pattern to preview
    pub fn preview(mut self) -> Self {
        self.pattern = self.pattern.display(ProductDisplayPattern::Preview);
        self
    }

    /// Set interaction state to focused
    pub fn focused(mut self) -> Self {
        self.pattern = self
            .pattern
            .interaction_state(ProductInteractionState::Focused);
        self
    }

    /// Set interaction state to selected
    pub fn selected(mut self) -> Self {
        self.pattern = self
            .pattern
            .interaction_state(ProductInteractionState::Selected);
        self
    }

    /// Set interaction state to loading
    pub fn loading(mut self) -> Self {
        self.pattern = self
            .pattern
            .interaction_state(ProductInteractionState::Loading);
        self
    }

    /// Set interaction state to disabled
    pub fn disabled(mut self) -> Self {
        self.pattern = self
            .pattern
            .interaction_state(ProductInteractionState::Disabled);
        self
    }

    /// Set availability to available
    pub fn available(mut self) -> Self {
        self.pattern = self
            .pattern
            .availability(ProductAvailabilityState::Available);
        self
    }

    /// Set availability to out of stock
    pub fn out_of_stock(mut self) -> Self {
        self.pattern = self
            .pattern
            .availability(ProductAvailabilityState::OutOfStock);
        self
    }

    /// Set availability to backorder
    pub fn backorder(mut self) -> Self {
        self.pattern = self
            .pattern
            .availability(ProductAvailabilityState::Backorder);
        self
    }

    /// Set availability to discontinued
    pub fn discontinued(mut self) -> Self {
        self.pattern = self
            .pattern
            .availability(ProductAvailabilityState::Discontinued);
        self
    }

    /// Set availability to limited
    pub fn limited(mut self) -> Self {
        self.pattern = self.pattern.availability(ProductAvailabilityState::Limited);
        self
    }

    /// Set prominence to subtle
    pub fn subtle(mut self) -> Self {
        self.pattern = self.pattern.prominence(ProductProminence::Subtle);
        self
    }

    /// Set prominence to standard
    pub fn standard(mut self) -> Self {
        self.pattern = self.pattern.prominence(ProductProminence::Standard);
        self
    }

    /// Set prominence to prominent
    pub fn prominent(mut self) -> Self {
        self.pattern = self.pattern.prominence(ProductProminence::Prominent);
        self
    }

    /// Set prominence to hero
    pub fn hero(mut self) -> Self {
        self.pattern = self.pattern.prominence(ProductProminence::Hero);
        self
    }

    /// Set image pattern to standard
    pub fn standard_image(mut self) -> Self {
        self.pattern = self.pattern.image_pattern(ProductImagePattern::Standard);
        self
    }

    /// Set image pattern to square
    pub fn square_image(mut self) -> Self {
        self.pattern = self.pattern.image_pattern(ProductImagePattern::Square);
        self
    }

    /// Set image pattern to wide
    pub fn wide_image(mut self) -> Self {
        self.pattern = self.pattern.image_pattern(ProductImagePattern::Wide);
        self
    }

    /// Set image pattern to portrait
    pub fn portrait_image(mut self) -> Self {
        self.pattern = self.pattern.image_pattern(ProductImagePattern::Portrait);
        self
    }

    /// Set image pattern to circle
    pub fn circle_image(mut self) -> Self {
        self.pattern = self.pattern.image_pattern(ProductImagePattern::Circle);
        self
    }

    /// Set information sections to basic
    pub fn basic_info(mut self) -> Self {
        self.pattern = self.pattern.info_sections(vec![ProductInfoSection::Basic]);
        self
    }

    /// Set information sections to extended
    pub fn extended_info(mut self) -> Self {
        self.pattern = self
            .pattern
            .info_sections(vec![ProductInfoSection::Extended]);
        self
    }

    /// Set information sections to detailed
    pub fn detailed_info(mut self) -> Self {
        self.pattern = self
            .pattern
            .info_sections(vec![ProductInfoSection::Detailed]);
        self
    }

    /// Set information sections to minimal
    pub fn minimal_info(mut self) -> Self {
        self.pattern = self
            .pattern
            .info_sections(vec![ProductInfoSection::Minimal]);
        self
    }

    /// Set price pattern to standard
    pub fn standard_price(mut self) -> Self {
        self.pattern = self.pattern.price_pattern(ProductPricePattern::Standard);
        self
    }

    /// Set price pattern to with compare
    pub fn price_with_compare(mut self) -> Self {
        self.pattern = self.pattern.price_pattern(ProductPricePattern::WithCompare);
        self
    }

    /// Set price pattern to range
    pub fn price_range(mut self) -> Self {
        self.pattern = self.pattern.price_pattern(ProductPricePattern::Range);
        self
    }

    /// Set price pattern to with discount
    pub fn price_with_discount(mut self) -> Self {
        self.pattern = self
            .pattern
            .price_pattern(ProductPricePattern::WithDiscount);
        self
    }

    /// Set price pattern to on sale
    pub fn price_on_sale(mut self) -> Self {
        self.pattern = self.pattern.price_pattern(ProductPricePattern::OnSale);
        self
    }

    /// Add action to add to cart
    pub fn add_to_cart_action(mut self) -> Self {
        let mut actions = self.pattern.actions.clone();
        if !actions.contains(&ProductActionType::AddToCart) {
            actions.push(ProductActionType::AddToCart);
        }
        self.pattern = self.pattern.actions(actions);
        self
    }

    /// Add action to quick view
    pub fn quick_view_action(mut self) -> Self {
        let mut actions = self.pattern.actions.clone();
        if !actions.contains(&ProductActionType::QuickView) {
            actions.push(ProductActionType::QuickView);
        }
        self.pattern = self.pattern.actions(actions);
        self
    }

    /// Add action to compare
    pub fn compare_action(mut self) -> Self {
        let mut actions = self.pattern.actions.clone();
        if !actions.contains(&ProductActionType::Compare) {
            actions.push(ProductActionType::Compare);
        }
        self.pattern = self.pattern.actions(actions);
        self
    }

    /// Add action to wishlist
    pub fn wishlist_action(mut self) -> Self {
        let mut actions = self.pattern.actions.clone();
        if !actions.contains(&ProductActionType::Wishlist) {
            actions.push(ProductActionType::Wishlist);
        }
        self.pattern = self.pattern.actions(actions);
        self
    }

    /// Add action to share
    pub fn share_action(mut self) -> Self {
        let mut actions = self.pattern.actions.clone();
        if !actions.contains(&ProductActionType::Share) {
            actions.push(ProductActionType::Share);
        }
        self.pattern = self.pattern.actions(actions);
        self
    }

    /// Add action to view details
    pub fn view_details_action(mut self) -> Self {
        let mut actions = self.pattern.actions.clone();
        if !actions.contains(&ProductActionType::ViewDetails) {
            actions.push(ProductActionType::ViewDetails);
        }
        self.pattern = self.pattern.actions(actions);
        self
    }

    /// Add sale badge
    pub fn sale_badge(mut self) -> Self {
        let mut badges = self.pattern.badges.clone();
        if !badges.contains(&ProductBadgeType::Sale) {
            badges.push(ProductBadgeType::Sale);
        }
        self.pattern = self.pattern.badges(badges);
        self
    }

    /// Add new badge
    pub fn new_badge(mut self) -> Self {
        let mut badges = self.pattern.badges.clone();
        if !badges.contains(&ProductBadgeType::New) {
            badges.push(ProductBadgeType::New);
        }
        self.pattern = self.pattern.badges(badges);
        self
    }

    /// Add featured badge
    pub fn featured_badge(mut self) -> Self {
        let mut badges = self.pattern.badges.clone();
        if !badges.contains(&ProductBadgeType::Featured) {
            badges.push(ProductBadgeType::Featured);
        }
        self.pattern = self.pattern.badges(badges);
        self
    }

    /// Add best seller badge
    pub fn best_seller_badge(mut self) -> Self {
        let mut badges = self.pattern.badges.clone();
        if !badges.contains(&ProductBadgeType::BestSeller) {
            badges.push(ProductBadgeType::BestSeller);
        }
        self.pattern = self.pattern.badges(badges);
        self
    }

    /// Add limited badge
    pub fn limited_badge(mut self) -> Self {
        let mut badges = self.pattern.badges.clone();
        if !badges.contains(&ProductBadgeType::Limited) {
            badges.push(ProductBadgeType::Limited);
        }
        self.pattern = self.pattern.badges(badges);
        self
    }

    /// Add out of stock badge
    pub fn out_of_stock_badge(mut self) -> Self {
        let mut badges = self.pattern.badges.clone();
        if !badges.contains(&ProductBadgeType::OutOfStock) {
            badges.push(ProductBadgeType::OutOfStock);
        }
        self.pattern = self.pattern.badges(badges);
        self
    }

    /// Add custom badge
    pub fn custom_badge(mut self, text: String) -> Self {
        let mut badges = self.pattern.badges.clone();
        badges.push(ProductBadgeType::Custom(text));
        self.pattern = self.pattern.badges(badges);
        self
    }

    /// Set variant pattern to dropdown
    pub fn dropdown_variants(mut self) -> Self {
        self.pattern = self
            .pattern
            .variant_pattern(Some(ProductVariantPattern::Dropdown));
        self
    }

    /// Set variant pattern to buttons
    pub fn button_variants(mut self) -> Self {
        self.pattern = self
            .pattern
            .variant_pattern(Some(ProductVariantPattern::Buttons));
        self
    }

    /// Set variant pattern to swatches
    pub fn swatch_variants(mut self) -> Self {
        self.pattern = self
            .pattern
            .variant_pattern(Some(ProductVariantPattern::Swatches));
        self
    }

    /// Set variant pattern to list
    pub fn list_variants(mut self) -> Self {
        self.pattern = self
            .pattern
            .variant_pattern(Some(ProductVariantPattern::List));
        self
    }

    /// Set variant pattern to radio
    pub fn radio_variants(mut self) -> Self {
        self.pattern = self
            .pattern
            .variant_pattern(Some(ProductVariantPattern::Radio));
        self
    }

    /// Add custom CSS class
    pub fn custom_class(mut self, class: String) -> Self {
        self.custom_classes.push(class);
        self
    }

    /// Generate CSS classes for product component
    pub fn classes(self) -> String {
        let mut classes = self.pattern.classes(self.colors);

        // Add custom classes
        if !self.custom_classes.is_empty() {
            classes.push(' ');
            classes.push_str(&self.custom_classes.join(" "));
        }

        classes
    }

    /// Generate CSS classes for product container
    pub fn container_classes(self) -> String {
        let base_classes = self.pattern.classes(self.colors);
        let padding = self.pattern.suggested_container_padding();
        let spacing = self.pattern.suggested_spacing();

        format!("{} {} {}", base_classes, padding, spacing)
    }

    /// Generate CSS classes for product image
    pub fn image_classes(self) -> String {
        let base_classes = "product-image";
        let aspect_ratio = self.pattern.suggested_image_aspect_ratio();
        let sizes = self.pattern.suggested_image_sizes();

        format!("{} {} {}", base_classes, aspect_ratio, sizes)
    }

    /// Generate CSS classes for product info section
    pub fn info_classes(self) -> String {
        let base_classes = "product-info";
        let spacing = self.pattern.suggested_spacing();

        format!("{} {}", base_classes, spacing)
    }

    /// Generate CSS classes for product actions
    pub fn actions_classes(self) -> String {
        let base_classes = "product-actions";
        let spacing = match self.pattern.display {
            ProductDisplayPattern::Tile => "gap-2",
            ProductDisplayPattern::Preview => "gap-1",
            _ => "gap-3",
        };

        format!("{} flex items-center {}", base_classes, spacing)
    }

    /// Generate CSS classes for product badges
    pub fn badges_classes(self) -> String {
        let base_classes = "product-badges";
        let positioning = "absolute top-2 right-2 flex flex-col gap-1";

        format!("{} {}", base_classes, positioning)
    }

    /// Get pattern configuration
    pub fn pattern(&self) -> &ProductCardPattern {
        &self.pattern
    }

    /// Get color provider
    pub fn colors(&self) -> &C {
        &self.colors
    }

    /// Get custom classes
    pub fn custom_classes(&self) -> &[String] {
        &self.custom_classes
    }
}

/// Convenience function for creating product component CSS classes
pub fn product_styles<C: ColorProvider>(colors: C) -> ProductBuilder<C> {
    ProductBuilder::new(colors)
}

/// Convenience function for creating featured product CSS classes
pub fn featured_product_styles<C: ColorProvider>(colors: C) -> ProductBuilder<C> {
    ProductBuilder::new(colors).featured().prominent()
}

/// Convenience function for creating product tile CSS classes
pub fn product_tile_styles<C: ColorProvider>(colors: C) -> ProductBuilder<C> {
    ProductBuilder::new(colors).tile().basic_info()
}

/// Convenience function for creating product showcase CSS classes
pub fn product_showcase_styles<C: ColorProvider>(colors: C) -> ProductBuilder<C> {
    ProductBuilder::new(colors).showcase().detailed_info()
}

/// Convenience function for creating product preview CSS classes
pub fn product_preview_styles<C: ColorProvider>(colors: C) -> ProductBuilder<C> {
    ProductBuilder::new(colors).preview().minimal_info()
}
