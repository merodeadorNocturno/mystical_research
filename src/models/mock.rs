use crate::models::index::{HeaderData, NavLink};
use crate::models::index::{Index, IndexBody, IndexFeaturedSection, IndexSchemaMarkup};

pub fn mock_index_schema_markup() -> IndexSchemaMarkup {
    IndexSchemaMarkup {
        site_name: "My Awesome Site".to_string(),
        site_description: "This is a description of my awesome site for search engines."
            .to_string(),
        main_image_url: Some("https://example.com/images/main-image.jpg".to_string()),
        search_term_string: Some("awesome, site, keywords".to_string()),
        canonical_url: Some("https://example.com/".to_string()),
    }
}

pub fn mock_index_body() -> IndexBody {
    IndexBody {
        title: "Welcome to My Awesome Site!".to_string(),
        description: "Explore the amazing features and content we have to offer.".to_string(),
        explore_url: "/explore".to_string(),
        learn_more_url: "/learn-more".to_string(),
        explore_label: "Discover More".to_string(),
    }
}

pub fn mock_index_featured_section(image_url: String) -> IndexFeaturedSection {
    IndexFeaturedSection {
        section_title: "Featured Section Title".to_string(),
        section_description: "This is a brief description of the featured section.".to_string(),
        section_content: "This is the main content of the featured section. You can put more detailed information here.".to_string(),
        section_image_url: Some(format!("/static/img/{image_url}")),
        section_alt_text: Some("Image for the featured section".to_string()),
        section_link_url: Some("/featured-link".to_string()),
        section_id: "featured-1".to_string(),
    }
}

pub fn mock_index() -> Index {
    Index {
        url: "/".to_string(),
        schema_markup: mock_index_schema_markup(),
    }
}

pub fn mock_header_data() -> HeaderData {
    HeaderData {
        site_name: "Angels Project".to_string(),
        site_description: "Empowering Innovation".to_string(),
        navigation_links: vec![
            NavLink {
                label: "Home".to_string(),
                url: "/".to_string(),
            },
            NavLink {
                label: "Explore".to_string(),
                url: "/explore".to_string(),
            },
            NavLink {
                label: "About Us".to_string(),
                url: "/about".to_string(),
            },
            NavLink {
                label: "Services".to_string(),
                url: "/services".to_string(),
            },
            NavLink {
                label: "Contact".to_string(),
                url: "/contact".to_string(),
            },
        ],
        logo_url: Some("/static/images/angels-logo-header.png".to_string()), // Example logo URL
    }
}
