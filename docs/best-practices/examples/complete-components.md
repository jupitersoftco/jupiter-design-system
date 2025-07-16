# Complete Component Examples

This guide showcases production-ready component implementations using the Jupiter Design System, demonstrating real-world patterns and best practices.

## 🎯 Overview

Learn how to build complete, production-ready components:
- Complex state management
- Accessibility integration
- Performance optimization
- Error handling
- Responsive design

## 🃏 Advanced Card Components

### Product Card Component

```rust
use jupiter_design_system::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub image_url: String,
    pub in_stock: bool,
    pub rating: f32,
    pub review_count: u32,
}

#[derive(Debug, Clone)]
pub struct ProductCardClasses {
    pub container: String,
    pub image_container: String,
    pub content: String,
    pub title: String,
    pub description: String,
    pub price_container: String,
    pub price: String,
    pub original_price: String,
    pub rating_container: String,
    pub rating_stars: String,
    pub rating_count: String,
    pub badge: String,
    pub button_container: String,
    pub primary_button: String,
    pub secondary_button: String,
    pub out_of_stock_overlay: String,
}

impl ProductCardClasses {
    pub fn new(colors: impl ColorProvider, is_featured: bool, is_interactive: bool) -> Self {
        let card_elevation = if is_featured { 
            CardElevation::Medium 
        } else { 
            CardElevation::Low 
        };
        
        let card_interaction = if is_interactive {
            CardInteraction::Clickable
        } else {
            CardInteraction::None
        };
        
        Self {
            container: card_styles(colors.clone())
                .elevation(card_elevation)
                .interaction(card_interaction)
                .spacing(CardSpacing::None)
                .classes() + " overflow-hidden relative group",
            
            image_container: layout_styles(colors.clone())
                .spacing_none()
                .classes() + " relative aspect-square overflow-hidden bg-gray-100",
            
            content: layout_styles(colors.clone())
                .direction_vertical()
                .spacing_md()
                .classes() + " p-4",
            
            title: text_styles(colors.clone())
                .typography(Typography::Heading3)
                .color(Color::TextPrimary)
                .weight(FontWeight::SemiBold)
                .classes() + " line-clamp-2 group-hover:text-primary transition-colors",
            
            description: text_styles(colors.clone())
                .typography(Typography::BodySmall)
                .color(Color::TextSecondary)
                .classes() + " line-clamp-3",
            
            price_container: layout_styles(colors.clone())
                .direction_horizontal()
                .spacing_sm()
                .alignment_start()
                .classes() + " items-baseline",
            
            price: text_styles(colors.clone())
                .typography(Typography::Body)
                .color(Color::TextPrimary)
                .weight(FontWeight::Bold)
                .classes(),
            
            original_price: text_styles(colors.clone())
                .typography(Typography::BodySmall)
                .color(Color::TextTertiary)
                .classes() + " line-through",
            
            rating_container: layout_styles(colors.clone())
                .direction_horizontal()
                .spacing_xs()
                .alignment_center()
                .classes(),
            
            rating_stars: text_styles(colors.clone())
                .typography(Typography::Caption)
                .color(Color::Warning)
                .classes(),
            
            rating_count: text_styles(colors.clone())
                .typography(Typography::Caption)
                .color(Color::TextTertiary)
                .classes(),
            
            badge: text_styles(colors.clone())
                .typography(Typography::Caption)
                .color(Color::TextInverse)
                .weight(FontWeight::Medium)
                .classes() + " px-2 py-1 rounded-full bg-primary text-xs uppercase tracking-wide",
            
            button_container: layout_styles(colors.clone())
                .direction_vertical()
                .spacing_sm()
                .classes() + " pt-2 border-t border-gray-100",
            
            primary_button: button_styles(colors.clone())
                .primary()
                .size(Size::Small)
                .full_width()
                .classes(),
            
            secondary_button: button_styles(colors.clone())
                .ghost()
                .size(Size::Small)
                .full_width()
                .classes(),
            
            out_of_stock_overlay: layout_styles(colors)
                .direction_vertical()
                .spacing_sm()
                .alignment_center()
                .classes() + " absolute inset-0 bg-white bg-opacity-90 flex items-center justify-center",
        }
    }
}

// Usage example with accessibility
pub fn render_product_card(product: &Product, colors: impl ColorProvider) -> String {
    let is_on_sale = product.price < 100.0; // Example sale logic
    let classes = ProductCardClasses::new(colors, is_on_sale, true);
    
    let badge_html = if is_on_sale {
        format!(r#"<div class="{}" aria-label="On Sale">Sale</div>"#, classes.badge)
    } else if product.rating > 4.5 {
        format!(r#"<div class="{}" aria-label="Bestseller">Bestseller</div>"#, classes.badge)
    } else {
        String::new()
    };
    
    let stock_overlay = if !product.in_stock {
        format!(
            r#"<div class="{}" aria-hidden="true">
                <div class="{}">Out of Stock</div>
                <div class="{}">Notify me when available</div>
               </div>"#,
            classes.out_of_stock_overlay,
            text_styles(colors.clone()).typography(Typography::Body).weight(FontWeight::Bold).classes(),
            text_styles(colors.clone()).typography(Typography::Caption).color(Color::TextSecondary).classes()
        )
    } else {
        String::new()
    };
    
    let rating_stars = "★".repeat(product.rating.floor() as usize) + 
                      &"☆".repeat(5 - product.rating.floor() as usize);
    
    format!(
        r#"<article class="{}" 
                   role="button" 
                   tabindex="0"
                   aria-labelledby="product-{}-title"
                   aria-describedby="product-{}-description product-{}-price">
            {badge}
            <div class="{}">
                <img src="{}" 
                     alt="{}" 
                     class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-300"
                     loading="lazy" />
            </div>
            <div class="{}">
                <h3 id="product-{}-title" class="{}">{}</h3>
                <p id="product-{}-description" class="{}">{}</p>
                
                <div class="{}">
                    <span class="{}" aria-hidden="true">{}</span>
                    <span class="{}">{} reviews</span>
                </div>
                
                <div class="{}">
                    <span id="product-{}-price" class="{}" aria-label="Price ${:.2}">${:.2}</span>
                    <span class="{}" aria-label="Original price ${:.2}">${:.2}</span>
                </div>
                
                <div class="{}">
                    <button class="{}" 
                            type="button"
                            {}
                            aria-describedby="product-{}-stock">
                        {}
                    </button>
                    <button class="{}" type="button">Add to Wishlist</button>
                </div>
            </div>
            {stock_overlay}
           </article>"#,
        classes.container,
        product.id, product.id, product.id,
        badge_html,
        classes.image_container,
        product.image_url, product.name,
        classes.content,
        product.id, classes.title, product.name,
        product.id, classes.description, product.description,
        classes.rating_container,
        classes.rating_stars, rating_stars,
        classes.rating_count, product.review_count,
        classes.price_container,
        product.id, classes.price, product.price, product.price,
        classes.original_price, product.price + 20.0, product.price + 20.0,
        classes.button_container,
        classes.primary_button,
        if product.in_stock { "" } else { r#"disabled aria-disabled="true""# },
        product.id,
        if product.in_stock { "Add to Cart" } else { "Out of Stock" },
        classes.secondary_button,
        stock_overlay
    )
}
```

### Modal Dialog Component

```rust
#[derive(Debug, Clone)]
pub struct ModalClasses {
    pub overlay: String,
    pub container: String,
    pub content: String,
    pub header: String,
    pub title: String,
    pub close_button: String,
    pub body: String,
    pub footer: String,
    pub primary_action: String,
    pub secondary_action: String,
}

impl ModalClasses {
    pub fn new(colors: impl ColorProvider, size: ModalSize) -> Self {
        let (max_width, spacing) = match size {
            ModalSize::Small => ("max-w-sm", CardSpacing::Medium),
            ModalSize::Medium => ("max-w-md", CardSpacing::Large),
            ModalSize::Large => ("max-w-lg", CardSpacing::Large),
            ModalSize::ExtraLarge => ("max-w-2xl", CardSpacing::XLarge),
        };
        
        Self {
            overlay: "fixed inset-0 bg-black bg-opacity-50 backdrop-blur-sm z-40 flex items-center justify-center p-4".to_string(),
            
            container: format!(
                "{} {} fixed z-50 w-full",
                card_styles(colors.clone())
                    .elevation(CardElevation::High)
                    .spacing(CardSpacing::None)
                    .classes(),
                max_width
            ),
            
            content: layout_styles(colors.clone())
                .direction_vertical()
                .spacing_none()
                .classes() + " max-h-screen overflow-auto",
            
            header: layout_styles(colors.clone())
                .direction_horizontal()
                .spacing_none()
                .alignment_between()
                .divider_bottom()
                .classes() + " p-6 pb-4",
            
            title: text_styles(colors.clone())
                .typography(Typography::Heading2)
                .color(Color::TextPrimary)
                .weight(FontWeight::SemiBold)
                .classes(),
            
            close_button: button_styles(colors.clone())
                .ghost()
                .size(Size::Small)
                .classes() + " p-1",
            
            body: layout_styles(colors.clone())
                .direction_vertical()
                .spacing_md()
                .classes() + " p-6 py-4",
            
            footer: layout_styles(colors.clone())
                .direction_horizontal()
                .spacing_sm()
                .alignment_end()
                .divider_top()
                .classes() + " p-6 pt-4",
            
            primary_action: button_styles(colors.clone())
                .primary()
                .size(Size::Medium)
                .classes(),
            
            secondary_action: button_styles(colors)
                .secondary()
                .size(Size::Medium)
                .classes(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ModalSize {
    Small,
    Medium,
    Large,
    ExtraLarge,
}

pub fn render_confirmation_modal(
    title: &str,
    message: &str,
    confirm_text: &str,
    cancel_text: &str,
    is_destructive: bool,
    colors: impl ColorProvider,
) -> String {
    let classes = ModalClasses::new(colors.clone(), ModalSize::Medium);
    
    let primary_button = if is_destructive {
        button_styles(colors).error().size(Size::Medium).classes()
    } else {
        classes.primary_action
    };
    
    format!(
        r#"<div class="{}" 
               role="dialog" 
               aria-modal="true" 
               aria-labelledby="modal-title"
               aria-describedby="modal-description"
               data-modal-overlay>
            <div class="{}" data-modal-content>
                <div class="{}">
                    <div class="{}">
                        <h2 id="modal-title" class="{}">{}</h2>
                        <button class="{}" 
                                type="button"
                                aria-label="Close dialog"
                                data-modal-close>
                            <span aria-hidden="true" class="text-xl">&times;</span>
                        </button>
                    </div>
                </div>
                
                <div class="{}">
                    <p id="modal-description" class="{}">{}</p>
                </div>
                
                <div class="{}">
                    <button class="{}" type="button" data-modal-close>
                        {}
                    </button>
                    <button class="{}" type="button" data-modal-confirm>
                        {}
                    </button>
                </div>
            </div>
           </div>"#,
        classes.overlay,
        classes.container,
        classes.content,
        classes.header,
        classes.title, title,
        classes.close_button,
        classes.body,
        text_styles(colors).typography(Typography::Body).color(Color::TextSecondary).classes(), message,
        classes.footer,
        classes.secondary_action, cancel_text,
        primary_button, confirm_text
    )
}
```

## 📊 Data Table Component

```rust
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TableClasses {
    pub container: String,
    pub table: String,
    pub header: String,
    pub header_row: String,
    pub header_cell: String,
    pub sortable_header: String,
    pub body: String,
    pub row: String,
    pub row_hover: String,
    pub row_selected: String,
    pub cell: String,
    pub pagination: String,
    pub pagination_info: String,
    pub pagination_controls: String,
}

impl TableClasses {
    pub fn new(colors: impl ColorProvider) -> Self {
        Self {
            container: card_styles(colors.clone())
                .elevation(CardElevation::Low)
                .spacing(CardSpacing::None)
                .classes() + " overflow-hidden",
            
            table: "w-full border-collapse".to_string(),
            
            header: colors.bg_class(Color::Surface) + " border-b " + &colors.border_class(Color::Border),
            
            header_row: "".to_string(),
            
            header_cell: text_styles(colors.clone())
                .typography(Typography::Label)
                .color(Color::TextPrimary)
                .weight(FontWeight::SemiBold)
                .classes() + " px-6 py-3 text-left",
            
            sortable_header: interactive_element(colors.clone())
                .base("px-6 py-3 text-left cursor-pointer select-none")
                .hover().bg_color(Color::Background)
                .focus().ring_color(Color::Interactive).outline_none()
                .build(),
            
            body: "divide-y " + &colors.border_class(Color::Border),
            
            row: "".to_string(),
            
            row_hover: "hover:" + &colors.bg_class(Color::Background) + " transition-colors",
            
            row_selected: colors.bg_class(Color::Interactive) + " bg-opacity-10",
            
            cell: text_styles(colors.clone())
                .typography(Typography::Body)
                .color(Color::TextSecondary)
                .classes() + " px-6 py-4",
            
            pagination: layout_styles(colors.clone())
                .direction_horizontal()
                .spacing_md()
                .alignment_between()
                .divider_top()
                .classes() + " p-6",
            
            pagination_info: text_styles(colors.clone())
                .typography(Typography::BodySmall)
                .color(Color::TextTertiary)
                .classes(),
            
            pagination_controls: layout_styles(colors)
                .direction_horizontal()
                .spacing_sm()
                .alignment_center()
                .classes(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TableColumn {
    pub key: String,
    pub title: String,
    pub sortable: bool,
    pub width: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TableData {
    pub rows: Vec<HashMap<String, String>>,
    pub total_count: usize,
    pub current_page: usize,
    pub page_size: usize,
}

pub fn render_data_table(
    columns: &[TableColumn],
    data: &TableData,
    sort_column: Option<&str>,
    sort_direction: SortDirection,
    selected_rows: &[usize],
    colors: impl ColorProvider,
) -> String {
    let classes = TableClasses::new(colors.clone());
    
    // Render header
    let header_cells = columns.iter().enumerate().map(|(i, column)| {
        let cell_class = if column.sortable {
            &classes.sortable_header
        } else {
            &classes.header_cell
        };
        
        let sort_indicator = if Some(column.key.as_str()) == sort_column {
            match sort_direction {
                SortDirection::Ascending => " ↑",
                SortDirection::Descending => " ↓",
            }
        } else {
            ""
        };
        
        let width_attr = column.width.as_ref()
            .map(|w| format!(r#" style="width: {}""#, w))
            .unwrap_or_default();
        
        if column.sortable {
            format!(
                r#"<th class="{}" 
                       role="button" 
                       tabindex="0"
                       aria-sort="{}"
                       data-sort-key="{}"{}>{}{}</th>"#,
                cell_class,
                if Some(column.key.as_str()) == sort_column {
                    match sort_direction {
                        SortDirection::Ascending => "ascending",
                        SortDirection::Descending => "descending",
                    }
                } else {
                    "none"
                },
                column.key,
                width_attr,
                column.title,
                sort_indicator
            )
        } else {
            format!(
                r#"<th class="{}"{}>{}</th>"#,
                cell_class,
                width_attr,
                column.title
            )
        }
    }).collect::<Vec<_>>().join("\n");
    
    // Render rows
    let body_rows = data.rows.iter().enumerate().map(|(row_index, row)| {
        let is_selected = selected_rows.contains(&row_index);
        let row_class = format!(
            "{} {} {}",
            classes.row,
            classes.row_hover,
            if is_selected { &classes.row_selected } else { "" }
        );
        
        let cells = columns.iter().map(|column| {
            let value = row.get(&column.key).unwrap_or(&String::new());
            format!(r#"<td class="{}">{}</td>"#, classes.cell, value)
        }).collect::<Vec<_>>().join("\n");
        
        format!(
            r#"<tr class="{}" 
                   {}
                   data-row-index="{}">
                {}
               </tr>"#,
            row_class,
            if is_selected { r#"aria-selected="true""# } else { "" },
            row_index,
            cells
        )
    }).collect::<Vec<_>>().join("\n");
    
    // Pagination
    let start_item = (data.current_page - 1) * data.page_size + 1;
    let end_item = std::cmp::min(data.current_page * data.page_size, data.total_count);
    
    let pagination_info = format!(
        "Showing {} to {} of {} results",
        start_item, end_item, data.total_count
    );
    
    let prev_button = button_styles(colors.clone())
        .secondary()
        .size(Size::Small)
        .state(if data.current_page == 1 { ButtonState::Disabled } else { ButtonState::Default })
        .classes();
    
    let next_button = button_styles(colors.clone())
        .secondary()
        .size(Size::Small)
        .state(if data.current_page * data.page_size >= data.total_count { ButtonState::Disabled } else { ButtonState::Default })
        .classes();
    
    format!(
        r#"<div class="{}" role="table" aria-label="Data table">
            <table class="{}">
                <thead class="{}">
                    <tr class="{}">
                        {}
                    </tr>
                </thead>
                <tbody class="{}">
                    {}
                </tbody>
            </table>
            
            <div class="{}" role="navigation" aria-label="Table pagination">
                <div class="{}">{}</div>
                <div class="{}">
                    <button class="{}" 
                            type="button"
                            {}
                            aria-label="Previous page">
                        Previous
                    </button>
                    <span class="{}">Page {} of {}</span>
                    <button class="{}" 
                            type="button"
                            {}
                            aria-label="Next page">
                        Next
                    </button>
                </div>
            </div>
           </div>"#,
        classes.container,
        classes.table,
        classes.header,
        classes.header_row,
        header_cells,
        classes.body,
        body_rows,
        classes.pagination,
        classes.pagination_info, pagination_info,
        classes.pagination_controls,
        prev_button,
        if data.current_page == 1 { r#"disabled aria-disabled="true""# } else { "" },
        text_styles(colors.clone()).typography(Typography::BodySmall).color(Color::TextSecondary).classes(),
        data.current_page,
        (data.total_count + data.page_size - 1) / data.page_size,
        next_button,
        if data.current_page * data.page_size >= data.total_count { r#"disabled aria-disabled="true""# } else { "" }
    )
}

#[derive(Debug, Clone, Copy)]
pub enum SortDirection {
    Ascending,
    Descending,
}
```

## 📱 Responsive Navigation Component

```rust
#[derive(Debug, Clone)]
pub struct NavigationClasses {
    pub navbar: String,
    pub container: String,
    pub brand: String,
    pub menu_toggle: String,
    pub menu: String,
    pub menu_open: String,
    pub menu_item: String,
    pub menu_link: String,
    pub menu_link_active: String,
    pub dropdown: String,
    pub dropdown_trigger: String,
    pub dropdown_menu: String,
    pub dropdown_item: String,
    pub user_menu: String,
    pub avatar: String,
}

impl NavigationClasses {
    pub fn new(colors: impl ColorProvider, is_mobile_menu_open: bool) -> Self {
        Self {
            navbar: layout_styles(colors.clone())
                .direction_horizontal()
                .spacing_none()
                .alignment_between()
                .classes() + &format!(" {} border-b {}", 
                    colors.bg_class(Color::Surface),
                    colors.border_class(Color::Border)
                ) + " px-4 py-3 lg:px-6",
            
            container: layout_styles(colors.clone())
                .direction_horizontal()
                .spacing_none()
                .alignment_between()
                .classes() + " w-full max-w-7xl mx-auto",
            
            brand: text_styles(colors.clone())
                .typography(Typography::Heading3)
                .color(Color::Primary)
                .weight(FontWeight::Bold)
                .classes(),
            
            menu_toggle: button_styles(colors.clone())
                .ghost()
                .size(Size::Medium)
                .classes() + " lg:hidden",
            
            menu: layout_styles(colors.clone())
                .direction_vertical()
                .spacing_sm()
                .classes() + " absolute top-full left-0 right-0 bg-white border-b shadow-lg lg:relative lg:top-auto lg:flex lg:flex-row lg:space-x-8 lg:space-y-0 lg:bg-transparent lg:border-0 lg:shadow-none p-4 lg:p-0",
            
            menu_open: if is_mobile_menu_open { "block" } else { "hidden lg:flex" }.to_string(),
            
            menu_item: "".to_string(),
            
            menu_link: interactive_element(colors.clone())
                .base("block px-3 py-2 rounded-md text-sm font-medium")
                .hover().bg_color(Color::Background).text_color(Color::TextPrimary)
                .focus().ring_color(Color::Interactive).outline_none()
                .build(),
            
            menu_link_active: interactive_element(colors.clone())
                .base("block px-3 py-2 rounded-md text-sm font-medium")
                .bg_color(Color::Interactive)
                .text_color(Color::TextInverse)
                .hover().bg_color(Color::InteractiveHover)
                .focus().ring_color(Color::Interactive).ring_offset_2().outline_none()
                .build(),
            
            dropdown: "relative".to_string(),
            
            dropdown_trigger: button_styles(colors.clone())
                .ghost()
                .size(Size::Small)
                .classes() + " flex items-center space-x-1",
            
            dropdown_menu: card_styles(colors.clone())
                .elevation(CardElevation::High)
                .spacing(CardSpacing::Small)
                .classes() + " absolute top-full right-0 mt-1 w-48 py-1 z-50",
            
            dropdown_item: interactive_element(colors.clone())
                .base("block px-4 py-2 text-sm")
                .hover().bg_color(Color::Background)
                .focus().bg_color(Color::Background).outline_none()
                .build(),
            
            user_menu: layout_styles(colors.clone())
                .direction_horizontal()
                .spacing_sm()
                .alignment_center()
                .classes(),
            
            avatar: "w-8 h-8 rounded-full bg-gray-300 flex items-center justify-center".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NavItem {
    pub label: String,
    pub href: Option<String>,
    pub is_active: bool,
    pub children: Vec<NavItem>,
}

pub fn render_navigation(
    brand_name: &str,
    nav_items: &[NavItem],
    user_name: Option<&str>,
    is_mobile_menu_open: bool,
    colors: impl ColorProvider,
) -> String {
    let classes = NavigationClasses::new(colors.clone(), is_mobile_menu_open);
    
    // Render navigation items
    let nav_html = nav_items.iter().map(|item| {
        if item.children.is_empty() {
            // Regular nav item
            let link_class = if item.is_active {
                &classes.menu_link_active
            } else {
                &classes.menu_link
            };
            
            if let Some(href) = &item.href {
                format!(
                    r#"<li class="{}">
                        <a href="{}" 
                           class="{}"
                           {}>{}</a>
                       </li>"#,
                    classes.menu_item,
                    href,
                    link_class,
                    if item.is_active { r#"aria-current="page""# } else { "" },
                    item.label
                )
            } else {
                format!(
                    r#"<li class="{}">
                        <span class="{}">{}</span>
                       </li>"#,
                    classes.menu_item,
                    link_class,
                    item.label
                )
            }
        } else {
            // Dropdown nav item
            let dropdown_items = item.children.iter().map(|child| {
                if let Some(href) = &child.href {
                    format!(
                        r#"<a href="{}" 
                           class="{}" 
                           role="menuitem">{}</a>"#,
                        href,
                        classes.dropdown_item,
                        child.label
                    )
                } else {
                    format!(
                        r#"<span class="{}" role="menuitem">{}</span>"#,
                        classes.dropdown_item,
                        child.label
                    )
                }
            }).collect::<Vec<_>>().join("\n");
            
            format!(
                r#"<li class="{} {}">
                    <button class="{}" 
                            type="button"
                            aria-expanded="false"
                            aria-haspopup="true"
                            data-dropdown-trigger>
                        {}
                        <span aria-hidden="true">▼</span>
                    </button>
                    <div class="{} hidden" 
                         role="menu"
                         data-dropdown-menu>
                        {}
                    </div>
                   </li>"#,
                classes.menu_item,
                classes.dropdown,
                classes.dropdown_trigger,
                item.label,
                classes.dropdown_menu,
                dropdown_items
            )
        }
    }).collect::<Vec<_>>().join("\n");
    
    // User menu
    let user_menu_html = if let Some(name) = user_name {
        format!(
            r#"<div class="{}">
                <div class="{}">
                    <span class="text-sm font-medium" aria-hidden="true">
                        {}
                    </span>
                </div>
                <div class="{} {}">
                    <button class="{}" 
                            type="button"
                            aria-expanded="false"
                            aria-haspopup="true"
                            aria-label="User menu"
                            data-user-menu-trigger>
                        <div class="{}">
                            <span class="text-sm font-medium" aria-hidden="true">
                                {}
                            </span>
                        </div>
                    </button>
                    <div class="{} hidden" 
                         role="menu"
                         data-user-menu>
                        <a href="/profile" class="{}" role="menuitem">Profile</a>
                        <a href="/settings" class="{}" role="menuitem">Settings</a>
                        <hr class="border-gray-200 my-1" />
                        <button class="{}" role="menuitem" type="button">Sign Out</button>
                    </div>
                </div>
               </div>"#,
            classes.user_menu,
            text_styles(colors.clone()).typography(Typography::BodySmall).color(Color::TextSecondary).classes(),
            name,
            classes.dropdown,
            "relative",
            classes.dropdown_trigger,
            classes.avatar,
            &name.chars().next().unwrap_or('?').to_uppercase().to_string(),
            classes.dropdown_menu,
            classes.dropdown_item,
            classes.dropdown_item,
            classes.dropdown_item
        )
    } else {
        format!(
            r#"<div class="{}">
                <a href="/login" class="{}">Sign In</a>
                <a href="/register" class="{}">Sign Up</a>
               </div>"#,
            layout_styles(colors.clone()).direction_horizontal().spacing_sm().classes(),
            button_styles(colors.clone()).ghost().size(Size::Small).classes(),
            button_styles(colors).primary().size(Size::Small).classes()
        )
    };
    
    format!(
        r#"<nav class="{}" role="navigation" aria-label="Main navigation">
            <div class="{}">
                <div class="flex items-center">
                    <a href="/" class="{}" aria-label="Home">{}</a>
                </div>
                
                <button class="{}" 
                        type="button"
                        aria-expanded="{}"
                        aria-controls="mobile-menu"
                        aria-label="Toggle navigation menu"
                        data-mobile-menu-toggle>
                    <span class="sr-only">Open main menu</span>
                    <span aria-hidden="true">☰</span>
                </button>
                
                <div class="hidden lg:flex lg:items-center lg:space-x-8">
                    <ul class="flex space-x-8" role="menubar">
                        {}
                    </ul>
                    {}
                </div>
            </div>
            
            <div id="mobile-menu" 
                 class="{} {}"
                 {}
                 aria-hidden="{}">
                <ul class="space-y-1" role="menubar">
                    {}
                </ul>
                <div class="pt-4 border-t border-gray-200 mt-4">
                    {}
                </div>
            </div>
           </nav>"#,
        classes.navbar,
        classes.container,
        classes.brand, brand_name,
        classes.menu_toggle,
        is_mobile_menu_open,
        nav_html,
        user_menu_html,
        classes.menu,
        classes.menu_open,
        if is_mobile_menu_open { r#"data-mobile-menu-open="true""# } else { "" },
        !is_mobile_menu_open,
        nav_html,
        user_menu_html
    )
}
```

## 🔗 Related Examples

- [Interactive Patterns](./interactive-patterns.md) - Advanced interaction patterns
- [Accessibility Showcase](./accessibility-showcase.md) - Comprehensive accessibility examples
- [Performance Optimization](./performance-optimization.md) - Performance techniques
- [Framework Integration](./dioxus-integration.md) - Framework-specific implementations