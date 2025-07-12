/// Product Patterns - E-commerce semantic design abstractions
///
/// This module provides abstract patterns for product display, interaction,
/// and commerce behaviors, enabling consistent and semantic product components.
use crate::core::color::{Color, ColorProvider};

/// Abstract product display patterns
#[derive(Debug, Clone, PartialEq)]
pub enum ProductDisplayPattern {
    /// Standard product listing item
    ListItem,
    /// Featured product with enhanced visibility
    Featured,
    /// Compact product tile
    Tile,
    /// Detailed product showcase
    Showcase,
    /// Quick view product preview
    Preview,
}

/// Product interaction states
#[derive(Debug, Clone, PartialEq)]
pub enum ProductInteractionState {
    /// Default interactive state
    Default,
    /// Hover/focused state
    Focused,
    /// Active/selected state
    Selected,
    /// Loading state (adding to cart, etc.)
    Loading,
    /// Disabled state (out of stock, etc.)
    Disabled,
}

/// Product availability states
#[derive(Debug, Clone, PartialEq)]
pub enum ProductAvailabilityState {
    /// Product is available for purchase
    Available,
    /// Product is out of stock
    OutOfStock,
    /// Product is on backorder
    Backorder,
    /// Product is discontinued
    Discontinued,
    /// Product has limited availability
    Limited,
}

/// Product prominence levels
#[derive(Debug, Clone, PartialEq)]
pub enum ProductProminence {
    /// Subtle product display
    Subtle,
    /// Standard product display
    Standard,
    /// Prominent product display
    Prominent,
    /// Hero product display
    Hero,
}

/// Product image display patterns
#[derive(Debug, Clone, PartialEq)]
pub enum ProductImagePattern {
    /// Standard product image with aspect ratio
    Standard,
    /// Square product image
    Square,
    /// Wide aspect ratio image
    Wide,
    /// Portrait aspect ratio image
    Portrait,
    /// Circular product image
    Circle,
}

/// Product badge types
#[derive(Debug, Clone, PartialEq)]
pub enum ProductBadgeType {
    /// Sale/discount badge
    Sale,
    /// New product badge
    New,
    /// Featured product badge
    Featured,
    /// Best seller badge
    BestSeller,
    /// Limited time badge
    Limited,
    /// Out of stock badge
    OutOfStock,
    /// Custom badge with text
    Custom(String),
}

/// Product action types
#[derive(Debug, Clone, PartialEq)]
pub enum ProductActionType {
    /// Add to cart action
    AddToCart,
    /// Quick view action
    QuickView,
    /// Compare action
    Compare,
    /// Wishlist action
    Wishlist,
    /// Share action
    Share,
    /// View details action
    ViewDetails,
}

/// Product information sections
#[derive(Debug, Clone, PartialEq)]
pub enum ProductInfoSection {
    /// Basic product information (title, price)
    Basic,
    /// Extended information (description, variants)
    Extended,
    /// Detailed information (specs, reviews)
    Detailed,
    /// Minimal information (title only)
    Minimal,
}

/// Product price display patterns
#[derive(Debug, Clone, PartialEq)]
pub enum ProductPricePattern {
    /// Standard price display
    Standard,
    /// Price with compare-at price
    WithCompare,
    /// Price range for variants
    Range,
    /// Price with discount percentage
    WithDiscount,
    /// Price on sale
    OnSale,
}

/// Product variant display patterns
#[derive(Debug, Clone, PartialEq)]
pub enum ProductVariantPattern {
    /// Dropdown selector
    Dropdown,
    /// Button selector
    Buttons,
    /// Swatch selector (colors, materials)
    Swatches,
    /// List selector
    List,
    /// Radio selector
    Radio,
}

/// Product card patterns for different contexts
#[derive(Debug, Clone, PartialEq)]
pub struct ProductCardPattern {
    /// Display pattern
    pub display: ProductDisplayPattern,
    /// Interaction state
    pub interaction_state: ProductInteractionState,
    /// Availability state
    pub availability: ProductAvailabilityState,
    /// Prominence level
    pub prominence: ProductProminence,
    /// Image pattern
    pub image_pattern: ProductImagePattern,
    /// Information sections to show
    pub info_sections: Vec<ProductInfoSection>,
    /// Price display pattern
    pub price_pattern: ProductPricePattern,
    /// Available actions
    pub actions: Vec<ProductActionType>,
    /// Badges to display
    pub badges: Vec<ProductBadgeType>,
    /// Variant display pattern
    pub variant_pattern: Option<ProductVariantPattern>,
}

impl ProductCardPattern {
    /// Create a new product card pattern
    pub fn new() -> Self {
        Self {
            display: ProductDisplayPattern::ListItem,
            interaction_state: ProductInteractionState::Default,
            availability: ProductAvailabilityState::Available,
            prominence: ProductProminence::Standard,
            image_pattern: ProductImagePattern::Standard,
            info_sections: vec![ProductInfoSection::Basic],
            price_pattern: ProductPricePattern::Standard,
            actions: vec![ProductActionType::AddToCart],
            badges: vec![],
            variant_pattern: None,
        }
    }

    /// Set display pattern
    pub fn display(mut self, display: ProductDisplayPattern) -> Self {
        self.display = display;
        self
    }

    /// Set interaction state
    pub fn interaction_state(mut self, state: ProductInteractionState) -> Self {
        self.interaction_state = state;
        self
    }

    /// Set availability state
    pub fn availability(mut self, availability: ProductAvailabilityState) -> Self {
        self.availability = availability;
        self
    }

    /// Set prominence level
    pub fn prominence(mut self, prominence: ProductProminence) -> Self {
        self.prominence = prominence;
        self
    }

    /// Set image pattern
    pub fn image_pattern(mut self, pattern: ProductImagePattern) -> Self {
        self.image_pattern = pattern;
        self
    }

    /// Set information sections
    pub fn info_sections(mut self, sections: Vec<ProductInfoSection>) -> Self {
        self.info_sections = sections;
        self
    }

    /// Set price pattern
    pub fn price_pattern(mut self, pattern: ProductPricePattern) -> Self {
        self.price_pattern = pattern;
        self
    }

    /// Set actions
    pub fn actions(mut self, actions: Vec<ProductActionType>) -> Self {
        self.actions = actions;
        self
    }

    /// Set badges
    pub fn badges(mut self, badges: Vec<ProductBadgeType>) -> Self {
        self.badges = badges;
        self
    }

    /// Set variant pattern
    pub fn variant_pattern(mut self, pattern: Option<ProductVariantPattern>) -> Self {
        self.variant_pattern = pattern;
        self
    }

    /// Get CSS classes for the pattern
    pub fn classes<C: ColorProvider>(&self, colors: C) -> String {
        let mut classes = Vec::new();

        // Base product card classes
        classes.push("product-card".to_string());

        // Display pattern classes
        match self.display {
            ProductDisplayPattern::ListItem => classes.push("product-card--list".to_string()),
            ProductDisplayPattern::Featured => classes.push("product-card--featured".to_string()),
            ProductDisplayPattern::Tile => classes.push("product-card--tile".to_string()),
            ProductDisplayPattern::Showcase => classes.push("product-card--showcase".to_string()),
            ProductDisplayPattern::Preview => classes.push("product-card--preview".to_string()),
        }

        // Interaction state classes
        match self.interaction_state {
            ProductInteractionState::Default => {}
            ProductInteractionState::Focused => classes.push("product-card--focused".to_string()),
            ProductInteractionState::Selected => classes.push("product-card--selected".to_string()),
            ProductInteractionState::Loading => classes.push("product-card--loading".to_string()),
            ProductInteractionState::Disabled => classes.push("product-card--disabled".to_string()),
        }

        // Availability state classes
        match self.availability {
            ProductAvailabilityState::Available => {}
            ProductAvailabilityState::OutOfStock => {
                classes.push("product-card--out-of-stock".to_string())
            }
            ProductAvailabilityState::Backorder => {
                classes.push("product-card--backorder".to_string())
            }
            ProductAvailabilityState::Discontinued => {
                classes.push("product-card--discontinued".to_string())
            }
            ProductAvailabilityState::Limited => classes.push("product-card--limited".to_string()),
        }

        // Prominence classes
        match self.prominence {
            ProductProminence::Subtle => classes.push("product-card--subtle".to_string()),
            ProductProminence::Standard => {}
            ProductProminence::Prominent => classes.push("product-card--prominent".to_string()),
            ProductProminence::Hero => classes.push("product-card--hero".to_string()),
        }

        // Color classes based on availability and prominence
        if self.availability != ProductAvailabilityState::Available {
            classes.push(colors.text_class(Color::InteractiveDisabled));
        } else {
            match self.prominence {
                ProductProminence::Hero => classes.push(colors.bg_class(Color::Primary)),
                ProductProminence::Prominent => classes.push(colors.bg_class(Color::Secondary)),
                _ => classes.push(colors.bg_class(Color::Surface)),
            }
        }

        classes.join(" ")
    }

    /// Get suggested image aspect ratio
    pub fn suggested_image_aspect_ratio(&self) -> &'static str {
        match self.image_pattern {
            ProductImagePattern::Standard => "aspect-[4/3]",
            ProductImagePattern::Square => "aspect-square",
            ProductImagePattern::Wide => "aspect-[16/9]",
            ProductImagePattern::Portrait => "aspect-[3/4]",
            ProductImagePattern::Circle => "aspect-square rounded-full",
        }
    }

    /// Get suggested image sizes
    pub fn suggested_image_sizes(&self) -> &'static str {
        match self.display {
            ProductDisplayPattern::ListItem => "h-48 w-48",
            ProductDisplayPattern::Featured => "h-64 w-64",
            ProductDisplayPattern::Tile => "h-40 w-40",
            ProductDisplayPattern::Showcase => "h-80 w-80",
            ProductDisplayPattern::Preview => "h-32 w-32",
        }
    }

    /// Get suggested container padding
    pub fn suggested_container_padding(&self) -> &'static str {
        match self.display {
            ProductDisplayPattern::ListItem => "p-4",
            ProductDisplayPattern::Featured => "p-6",
            ProductDisplayPattern::Tile => "p-3",
            ProductDisplayPattern::Showcase => "p-8",
            ProductDisplayPattern::Preview => "p-2",
        }
    }

    /// Get suggested spacing between elements
    pub fn suggested_spacing(&self) -> &'static str {
        match self.display {
            ProductDisplayPattern::ListItem => "space-y-3",
            ProductDisplayPattern::Featured => "space-y-4",
            ProductDisplayPattern::Tile => "space-y-2",
            ProductDisplayPattern::Showcase => "space-y-6",
            ProductDisplayPattern::Preview => "space-y-1",
        }
    }
}

impl Default for ProductCardPattern {
    fn default() -> Self {
        Self::new()
    }
}
