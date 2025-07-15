#[cfg(test)]
mod tests {
    use super::*;
    use crate::patterns::product::*;
    use crate::themes::VibeColors;

    #[test]
    fn test_product_builder_new() {
        let colors = VibeColors::default();
        let builder = ProductBuilder::new(colors);

        assert_eq!(builder.pattern().display, ProductDisplayPattern::ListItem);
        assert_eq!(
            builder.pattern().interaction_state,
            ProductInteractionState::Default
        );
        assert_eq!(
            builder.pattern().availability,
            ProductAvailabilityState::Available
        );
        assert_eq!(builder.pattern().prominence, ProductProminence::Standard);
        assert_eq!(
            builder.pattern().image_pattern,
            ProductImagePattern::Standard
        );
        assert_eq!(
            builder.pattern().info_sections,
            vec![ProductInfoSection::Basic]
        );
        assert_eq!(
            builder.pattern().price_pattern,
            ProductPricePattern::Standard
        );
        assert_eq!(
            builder.pattern().actions,
            vec![ProductActionType::AddToCart]
        );
        assert_eq!(builder.pattern().badges, Vec::<ProductBadgeType>::new());
        assert_eq!(builder.pattern().variant_pattern, None);
        assert_eq!(builder.custom_classes().len(), 0);
    }

    #[test]
    fn test_display_patterns() {
        let colors = VibeColors::default();

        let list_item = ProductBuilder::new(colors.clone()).list_item();
        assert_eq!(list_item.pattern().display, ProductDisplayPattern::ListItem);

        let featured = ProductBuilder::new(colors.clone()).featured();
        assert_eq!(featured.pattern().display, ProductDisplayPattern::Featured);

        let tile = ProductBuilder::new(colors.clone()).tile();
        assert_eq!(tile.pattern().display, ProductDisplayPattern::Tile);

        let showcase = ProductBuilder::new(colors.clone()).showcase();
        assert_eq!(showcase.pattern().display, ProductDisplayPattern::Showcase);

        let preview = ProductBuilder::new(colors.clone()).preview();
        assert_eq!(preview.pattern().display, ProductDisplayPattern::Preview);
    }

    #[test]
    fn test_interaction_states() {
        let colors = VibeColors::default();

        let focused = ProductBuilder::new(colors.clone()).focused();
        assert_eq!(
            focused.pattern().interaction_state,
            ProductInteractionState::Focused
        );

        let selected = ProductBuilder::new(colors.clone()).selected();
        assert_eq!(
            selected.pattern().interaction_state,
            ProductInteractionState::Selected
        );

        let loading = ProductBuilder::new(colors.clone()).loading();
        assert_eq!(
            loading.pattern().interaction_state,
            ProductInteractionState::Loading
        );

        let disabled = ProductBuilder::new(colors.clone()).disabled();
        assert_eq!(
            disabled.pattern().interaction_state,
            ProductInteractionState::Disabled
        );
    }

    #[test]
    fn test_availability_states() {
        let colors = VibeColors::default();

        let available = ProductBuilder::new(colors.clone()).available();
        assert_eq!(
            available.pattern().availability,
            ProductAvailabilityState::Available
        );

        let out_of_stock = ProductBuilder::new(colors.clone()).out_of_stock();
        assert_eq!(
            out_of_stock.pattern().availability,
            ProductAvailabilityState::OutOfStock
        );

        let backorder = ProductBuilder::new(colors.clone()).backorder();
        assert_eq!(
            backorder.pattern().availability,
            ProductAvailabilityState::Backorder
        );

        let discontinued = ProductBuilder::new(colors.clone()).discontinued();
        assert_eq!(
            discontinued.pattern().availability,
            ProductAvailabilityState::Discontinued
        );

        let limited = ProductBuilder::new(colors.clone()).limited();
        assert_eq!(
            limited.pattern().availability,
            ProductAvailabilityState::Limited
        );
    }

    #[test]
    fn test_prominence_levels() {
        let colors = VibeColors::default();

        let subtle = ProductBuilder::new(colors.clone()).subtle();
        assert_eq!(subtle.pattern().prominence, ProductProminence::Subtle);

        let standard = ProductBuilder::new(colors.clone()).standard();
        assert_eq!(standard.pattern().prominence, ProductProminence::Standard);

        let prominent = ProductBuilder::new(colors.clone()).prominent();
        assert_eq!(prominent.pattern().prominence, ProductProminence::Prominent);

        let hero = ProductBuilder::new(colors.clone()).hero();
        assert_eq!(hero.pattern().prominence, ProductProminence::Hero);
    }

    #[test]
    fn test_image_patterns() {
        let colors = VibeColors::default();

        let standard = ProductBuilder::new(colors.clone()).standard_image();
        assert_eq!(
            standard.pattern().image_pattern,
            ProductImagePattern::Standard
        );

        let square = ProductBuilder::new(colors.clone()).square_image();
        assert_eq!(square.pattern().image_pattern, ProductImagePattern::Square);

        let wide = ProductBuilder::new(colors.clone()).wide_image();
        assert_eq!(wide.pattern().image_pattern, ProductImagePattern::Wide);

        let portrait = ProductBuilder::new(colors.clone()).portrait_image();
        assert_eq!(
            portrait.pattern().image_pattern,
            ProductImagePattern::Portrait
        );

        let circle = ProductBuilder::new(colors.clone()).circle_image();
        assert_eq!(circle.pattern().image_pattern, ProductImagePattern::Circle);
    }

    #[test]
    fn test_info_sections() {
        let colors = VibeColors::default();

        let basic = ProductBuilder::new(colors.clone()).basic_info();
        assert_eq!(
            basic.pattern().info_sections,
            vec![ProductInfoSection::Basic]
        );

        let extended = ProductBuilder::new(colors.clone()).extended_info();
        assert_eq!(
            extended.pattern().info_sections,
            vec![ProductInfoSection::Extended]
        );

        let detailed = ProductBuilder::new(colors.clone()).detailed_info();
        assert_eq!(
            detailed.pattern().info_sections,
            vec![ProductInfoSection::Detailed]
        );

        let minimal = ProductBuilder::new(colors.clone()).minimal_info();
        assert_eq!(
            minimal.pattern().info_sections,
            vec![ProductInfoSection::Minimal]
        );
    }

    #[test]
    fn test_price_patterns() {
        let colors = VibeColors::default();

        let standard = ProductBuilder::new(colors.clone()).standard_price();
        assert_eq!(
            standard.pattern().price_pattern,
            ProductPricePattern::Standard
        );

        let with_compare = ProductBuilder::new(colors.clone()).price_with_compare();
        assert_eq!(
            with_compare.pattern().price_pattern,
            ProductPricePattern::WithCompare
        );

        let range = ProductBuilder::new(colors.clone()).price_range();
        assert_eq!(range.pattern().price_pattern, ProductPricePattern::Range);

        let with_discount = ProductBuilder::new(colors.clone()).price_with_discount();
        assert_eq!(
            with_discount.pattern().price_pattern,
            ProductPricePattern::WithDiscount
        );

        let on_sale = ProductBuilder::new(colors.clone()).price_on_sale();
        assert_eq!(on_sale.pattern().price_pattern, ProductPricePattern::OnSale);
    }

    #[test]
    fn test_actions() {
        let colors = VibeColors::default();

        let add_to_cart = ProductBuilder::new(colors.clone()).add_to_cart_action();
        assert!(add_to_cart
            .pattern()
            .actions
            .contains(&ProductActionType::AddToCart));

        let quick_view = ProductBuilder::new(colors.clone()).quick_view_action();
        assert!(quick_view
            .pattern()
            .actions
            .contains(&ProductActionType::QuickView));

        let compare = ProductBuilder::new(colors.clone()).compare_action();
        assert!(compare
            .pattern()
            .actions
            .contains(&ProductActionType::Compare));

        let wishlist = ProductBuilder::new(colors.clone()).wishlist_action();
        assert!(wishlist
            .pattern()
            .actions
            .contains(&ProductActionType::Wishlist));

        let share = ProductBuilder::new(colors.clone()).share_action();
        assert!(share.pattern().actions.contains(&ProductActionType::Share));

        let view_details = ProductBuilder::new(colors.clone()).view_details_action();
        assert!(view_details
            .pattern()
            .actions
            .contains(&ProductActionType::ViewDetails));
    }

    #[test]
    fn test_badges() {
        let colors = VibeColors::default();

        let sale = ProductBuilder::new(colors.clone()).sale_badge();
        assert!(sale.pattern().badges.contains(&ProductBadgeType::Sale));

        let new = ProductBuilder::new(colors.clone()).new_badge();
        assert!(new.pattern().badges.contains(&ProductBadgeType::New));

        let featured = ProductBuilder::new(colors.clone()).featured_badge();
        assert!(featured
            .pattern()
            .badges
            .contains(&ProductBadgeType::Featured));

        let best_seller = ProductBuilder::new(colors.clone()).best_seller_badge();
        assert!(best_seller
            .pattern()
            .badges
            .contains(&ProductBadgeType::BestSeller));

        let limited = ProductBuilder::new(colors.clone()).limited_badge();
        assert!(limited
            .pattern()
            .badges
            .contains(&ProductBadgeType::Limited));

        let out_of_stock = ProductBuilder::new(colors.clone()).out_of_stock_badge();
        assert!(out_of_stock
            .pattern()
            .badges
            .contains(&ProductBadgeType::OutOfStock));

        let custom = ProductBuilder::new(colors.clone()).custom_badge("Custom".to_string());
        assert!(custom
            .pattern()
            .badges
            .contains(&ProductBadgeType::Custom("Custom".to_string())));
    }

    #[test]
    fn test_variant_patterns() {
        let colors = VibeColors::default();

        let dropdown = ProductBuilder::new(colors.clone()).dropdown_variants();
        assert_eq!(
            dropdown.pattern().variant_pattern,
            Some(ProductVariantPattern::Dropdown)
        );

        let buttons = ProductBuilder::new(colors.clone()).button_variants();
        assert_eq!(
            buttons.pattern().variant_pattern,
            Some(ProductVariantPattern::Buttons)
        );

        let swatches = ProductBuilder::new(colors.clone()).swatch_variants();
        assert_eq!(
            swatches.pattern().variant_pattern,
            Some(ProductVariantPattern::Swatches)
        );

        let list = ProductBuilder::new(colors.clone()).list_variants();
        assert_eq!(
            list.pattern().variant_pattern,
            Some(ProductVariantPattern::List)
        );

        let radio = ProductBuilder::new(colors.clone()).radio_variants();
        assert_eq!(
            radio.pattern().variant_pattern,
            Some(ProductVariantPattern::Radio)
        );
    }

    #[test]
    fn test_custom_classes() {
        let colors = VibeColors::default();

        let builder = ProductBuilder::new(colors)
            .custom_class("custom-1".to_string())
            .custom_class("custom-2".to_string());

        assert_eq!(builder.custom_classes().len(), 2);
        assert_eq!(builder.custom_classes()[0], "custom-1");
        assert_eq!(builder.custom_classes()[1], "custom-2");
    }

    #[test]
    fn test_chaining() {
        let colors = VibeColors::default();

        let builder = ProductBuilder::new(colors)
            .featured()
            .prominent()
            .square_image()
            .extended_info()
            .price_with_compare()
            .add_to_cart_action()
            .quick_view_action()
            .sale_badge()
            .new_badge()
            .button_variants()
            .custom_class("custom-product".to_string());

        assert_eq!(builder.pattern().display, ProductDisplayPattern::Featured);
        assert_eq!(builder.pattern().prominence, ProductProminence::Prominent);
        assert_eq!(builder.pattern().image_pattern, ProductImagePattern::Square);
        assert_eq!(
            builder.pattern().info_sections,
            vec![ProductInfoSection::Extended]
        );
        assert_eq!(
            builder.pattern().price_pattern,
            ProductPricePattern::WithCompare
        );
        assert!(builder
            .pattern()
            .actions
            .contains(&ProductActionType::AddToCart));
        assert!(builder
            .pattern()
            .actions
            .contains(&ProductActionType::QuickView));
        assert!(builder.pattern().badges.contains(&ProductBadgeType::Sale));
        assert!(builder.pattern().badges.contains(&ProductBadgeType::New));
        assert_eq!(
            builder.pattern().variant_pattern,
            Some(ProductVariantPattern::Buttons)
        );
        assert!(builder
            .custom_classes()
            .contains(&"custom-product".to_string()));
    }

    #[test]
    fn test_classes_generation() {
        let colors = VibeColors::default();

        let classes = ProductBuilder::new(colors).featured().prominent().classes();
        assert!(classes.contains("product-card"));
        assert!(classes.contains("product-card--featured"));
        assert!(classes.contains("product-card--prominent"));
    }

    #[test]
    fn test_container_classes() {
        let colors = VibeColors::default();

        let classes = ProductBuilder::new(colors.clone())
            .featured()
            .container_classes();
        assert!(classes.contains("product-card"));
        assert!(classes.contains("product-card--featured"));
        assert!(classes.contains("p-6")); // featured padding
        assert!(classes.contains("space-y-4")); // featured spacing
    }

    #[test]
    fn test_image_classes() {
        let colors = VibeColors::default();

        let classes = ProductBuilder::new(colors.clone())
            .featured()
            .square_image()
            .image_classes();
        assert!(classes.contains("product-image"));
        assert!(classes.contains("aspect-square"));
        assert!(classes.contains("h-64 w-64")); // featured size
    }

    #[test]
    fn test_info_classes() {
        let colors = VibeColors::default();

        let classes = ProductBuilder::new(colors.clone())
            .featured()
            .info_classes();
        assert!(classes.contains("product-info"));
        assert!(classes.contains("space-y-4")); // featured spacing
    }

    #[test]
    fn test_actions_classes() {
        let colors = VibeColors::default();

        let classes = ProductBuilder::new(colors.clone())
            .featured()
            .actions_classes();
        assert!(classes.contains("product-actions"));
        assert!(classes.contains("flex"));
        assert!(classes.contains("items-center"));
        assert!(classes.contains("gap-3"));
    }

    #[test]
    fn test_badges_classes() {
        let colors = VibeColors::default();

        let classes = ProductBuilder::new(colors.clone()).badges_classes();
        assert!(classes.contains("product-badges"));
        assert!(classes.contains("absolute"));
        assert!(classes.contains("top-2"));
        assert!(classes.contains("right-2"));
        assert!(classes.contains("flex"));
        assert!(classes.contains("flex-col"));
        assert!(classes.contains("gap-1"));
    }

    #[test]
    fn test_convenience_functions() {
        let colors = VibeColors::default();

        let product = product_styles(colors.clone());
        assert_eq!(product.pattern().display, ProductDisplayPattern::ListItem);

        let featured = featured_product_styles(colors.clone());
        assert_eq!(featured.pattern().display, ProductDisplayPattern::Featured);
        assert_eq!(featured.pattern().prominence, ProductProminence::Prominent);

        let tile = product_tile_styles(colors.clone());
        assert_eq!(tile.pattern().display, ProductDisplayPattern::Tile);
        assert_eq!(
            tile.pattern().info_sections,
            vec![ProductInfoSection::Basic]
        );

        let showcase = product_showcase_styles(colors.clone());
        assert_eq!(showcase.pattern().display, ProductDisplayPattern::Showcase);
        assert_eq!(
            showcase.pattern().info_sections,
            vec![ProductInfoSection::Detailed]
        );

        let preview = product_preview_styles(colors.clone());
        assert_eq!(preview.pattern().display, ProductDisplayPattern::Preview);
        assert_eq!(
            preview.pattern().info_sections,
            vec![ProductInfoSection::Minimal]
        );
    }

    #[test]
    fn test_duplicate_actions() {
        let colors = VibeColors::default();

        let builder = ProductBuilder::new(colors)
            .add_to_cart_action()
            .add_to_cart_action(); // Adding same action twice

        // Should only have one instance of AddToCart
        let count = builder
            .pattern()
            .actions
            .iter()
            .filter(|&action| action == &ProductActionType::AddToCart)
            .count();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_duplicate_badges() {
        let colors = VibeColors::default();

        let builder = ProductBuilder::new(colors).sale_badge().sale_badge(); // Adding same badge twice

        // Should only have one instance of Sale badge
        let count = builder
            .pattern()
            .badges
            .iter()
            .filter(|&badge| badge == &ProductBadgeType::Sale)
            .count();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_pattern_suggestions() {
        let colors = VibeColors::default();

        let pattern = ProductBuilder::new(colors).featured().pattern();

        assert_eq!(pattern.suggested_image_aspect_ratio(), "aspect-[4/3]");
        assert_eq!(pattern.suggested_image_sizes(), "h-64 w-64");
        assert_eq!(pattern.suggested_container_padding(), "p-6");
        assert_eq!(pattern.suggested_spacing(), "space-y-4");
    }

    #[test]
    fn test_out_of_stock_styling() {
        let colors = VibeColors::default();

        let classes = ProductBuilder::new(colors).out_of_stock().classes();
        assert!(classes.contains("product-card--out-of-stock"));
    }

    #[test]
    fn test_loading_state_styling() {
        let colors = VibeColors::default();

        let classes = ProductBuilder::new(colors).loading().classes();
        assert!(classes.contains("product-card--loading"));
    }

    #[test]
    fn test_multiple_info_sections() {
        let colors = VibeColors::default();

        let builder = ProductBuilder::new(colors).basic_info().extended_info(); // This should override the previous

        assert_eq!(
            builder.pattern().info_sections,
            vec![ProductInfoSection::Extended]
        );
    }

    #[test]
    fn test_variant_pattern_override() {
        let colors = VibeColors::default();

        let builder = ProductBuilder::new(colors)
            .dropdown_variants()
            .button_variants(); // This should override the previous

        assert_eq!(
            builder.pattern().variant_pattern,
            Some(ProductVariantPattern::Buttons)
        );
    }

    #[test]
    fn test_image_pattern_suggestions() {
        let colors = VibeColors::default();

        let standard = ProductBuilder::new(colors.clone())
            .standard_image()
            .pattern();
        assert_eq!(standard.suggested_image_aspect_ratio(), "aspect-[4/3]");

        let square = ProductBuilder::new(colors.clone()).square_image().pattern();
        assert_eq!(square.suggested_image_aspect_ratio(), "aspect-square");

        let wide = ProductBuilder::new(colors.clone()).wide_image().pattern();
        assert_eq!(wide.suggested_image_aspect_ratio(), "aspect-[16/9]");

        let portrait = ProductBuilder::new(colors.clone())
            .portrait_image()
            .pattern();
        assert_eq!(portrait.suggested_image_aspect_ratio(), "aspect-[3/4]");

        let circle = ProductBuilder::new(colors.clone()).circle_image().pattern();
        assert_eq!(
            circle.suggested_image_aspect_ratio(),
            "aspect-square rounded-full"
        );
    }

    #[test]
    fn test_actions_spacing_by_display() {
        let colors = VibeColors::default();

        let tile_classes = ProductBuilder::new(colors.clone()).tile().actions_classes();
        assert!(tile_classes.contains("gap-2"));

        let preview_classes = ProductBuilder::new(colors.clone())
            .preview()
            .actions_classes();
        assert!(preview_classes.contains("gap-1"));

        let standard_classes = ProductBuilder::new(colors.clone())
            .featured()
            .actions_classes();
        assert!(standard_classes.contains("gap-3"));
    }
}
