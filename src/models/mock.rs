use crate::models::{
    about_model::{AboutBody, AboutHomePage, AboutHomeSchemaMarkup, AboutItem, AboutStructure},
    ai_model::BlogStructure,
    blog_model::*,
    contact_models::{ContactHomePage, ContactHomeSchemaMarkup},
    general_model::PageType,
    index_model::{HeaderData, Index, IndexBody, IndexFeaturedSection, IndexSchemaMarkup, NavLink},
    topics_model::{TopicCategory, TopicHomePage, TopicHomeSchemaMarkup},
};

pub fn mock_index_schema_markup() -> IndexSchemaMarkup {
    IndexSchemaMarkup {
        site_name: "My Awesome Site".to_string(),
        site_description: "This is a description of my awesome site for search engines."
            .to_string(),
        main_image_url: Some("main-image.jpg".to_string()),
        search_term_string: Some("awesome, site, keywords".to_string()),
        canonical_url: Some("https://mysticalresearch.com/".to_string()),
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
                label: "About Us".to_string(),
                url: "/about.html".to_string(),
            },
            NavLink {
                label: "Topics".to_string(),
                url: "/topics.html".to_string(),
            },
            NavLink {
                label: "Resources".to_string(),
                url: "/resources.html".to_string(),
            },
            NavLink {
                label: "Blog".to_string(),
                url: "/blog.html".to_string(),
            },
            NavLink {
                label: "Contact".to_string(),
                url: "/contact.html".to_string(),
            },
        ],
        logo_url: Some("/static/images/angels-logo-header.png".to_string()), // Example logo URL
        canonical_url: Some("https://example.com/".to_string()), // Example canonical URL
    }
}

pub fn mock_blog_home_page_object() -> BlogHomePage {
    BlogHomePage {
        body: BlogBody {
            structure: BlogStructure {
                 title: "Blog Title".to_string(),
                 table_of_contents: vec!["Introduction".to_string(), "The Problem of Evil".to_string(), "Divine Command Theory".to_string(), "Justification of Violence".to_string(), "Religious Discrimination".to_string(), "Abstract Concepts".to_string()],
                 summary: "Blog Summary".to_string(),
                 content: "This article provides a critical overview of common ethical challenges that arise from theological doctrines and religious practices. We'll examine issues like the problem of evil, divine command theory, the justification of violence in the name of religion, and the potential for religious beliefs to be used to justify discrimination and oppression. Furthermore, we'll consider the inherent contradictions that can emerge when abstract theological concepts are applied to the complexities of everyday life, forcing a re-evaluation of traditional interpretations and demanding a more nuanced approach to ethical decision-making within religious frameworks.".to_string(),
                 keywords: vec!["Ethics".to_string(), "Theology".to_string(), "Religion".to_string()],
            }, // Using the mock BlogStructure
            author: "Mock Author Name".to_string(),
            image_urls: Some(vec!["https://example.com/images/body_image.png".to_string()]),
        },
        schema_markup: BlogHomeSchemaMarkup {
            blog_title: "Mock Blog Home Title".to_string(),
            blog_description: "Mock Blog Home Description".to_string(),
            blog_name: "Mock Blog Name".to_string(),
            blog_url: "https://example.com/blog".to_string(),
            blog_logo_url: "https://example.com/images/blog_logo.png".to_string(),
            blog_search_action: "/search?q={search_term_string}".to_string(),
        },
        featured: vec![
            BlogFeaturedSection {
                blog_id: "mock-blog-id-1".to_string(),
                blog_title: "Mock Featured Blog Title 1".to_string(),
                blog_description: "Mock Featured Blog Description 1".to_string(),
                blog_image_urls: Some(vec![
                    BlogImage {
                        blog_image_url: "https://example.com/images/featured1-image1.png"
                            .to_string(),
                        blog_image_width: Some("800".to_string()),
                        blog_image_height: "600".to_string(),
                        blog_image_alt: Some("Mock Featured Blog Image 1-1".to_string()),
                    },
                    BlogImage {
                        blog_image_url: "https://example.com/images/featured1-image2.png"
                            .to_string(),
                        blog_image_width: Some("400".to_string()),
                        blog_image_height: "300".to_string(),
                        blog_image_alt: Some("Mock Featured Blog Image 1-2".to_string()),
                    },
                ]),
            },
            BlogFeaturedSection {
                blog_id: "mock-blog-id-2".to_string(),
                blog_title: "Mock Featured Blog Title 2".to_string(),
                blog_description: "Mock Featured Blog Description 2".to_string(),
                blog_image_urls: Some(vec![BlogImage {
                    blog_image_url: "https://example.com/images/featured2-image.png".to_string(),
                    blog_image_width: Some("600".to_string()),
                    blog_image_height: "450".to_string(),
                    blog_image_alt: Some("Mock Featured Blog Image 2-1".to_string()),
                }]),
            },
        ],
        header: HeaderData {
            site_name: "Mock Site".to_string(),
            site_description: "Mock Site Description".to_string(),
            navigation_links: vec![
                NavLink {
                    label: "Home".to_string(),
                    url: "/".to_string(),
                },
                NavLink {
                    label: "Blog".to_string(),
                    url: "/blog".to_string(),
                },
                NavLink {
                    label: "About".to_string(),
                    url: "/about".to_string(),
                },
            ],
            logo_url: Some("https://example.com/images/logo.png".to_string()),
            canonical_url: Some("https://example.com/blog".to_string()),
        },
        section: PageType::BlogHome,
    }
}

pub fn mock_contact_home_page_object() -> ContactHomePage {
    ContactHomePage {
        header: mock_header_data(),
        hero_title: "Get in Touch".to_string(),
        hero_description: "We'd love to hear from you!".to_string(),
        contact_email: "info@mysticweb.com".to_string(),
        contact_phone: "+1-555-MYSTIC".to_string(),
        contact_address: "123 Celestial Way\nMystic Falls, CA 90001".to_string(),
        schema_markup: ContactHomeSchemaMarkup {
            contact_page_title: "Contact Us - Mystical Research".to_string(),
            site_description: "Get in touch with us for any inquiries or support.".to_string(),
            canonical_url: "https://mysticalresearch.com/contact.html".to_string(),
            site_name: "Mystical Research".to_string(),
            site_logo_url: "https://mysticalresearch.com/images/logo.png".to_string(),
            search_action_target: "https://mysticalresearch.com/search?q={search_term_string}"
                .to_string(),
        },
        section: PageType::Contact,
    }
}

pub fn mock_topic_home_page_object() -> TopicHomePage {
    TopicHomePage {
        header: mock_header_data(),
        schema_markup: TopicHomeSchemaMarkup {
            topic_page_title: "Explore Mystical Topics - Angels Project".to_string(),
            site_description: "Dive into a wide range of mystical and spiritual topics."
                .to_string(),
            canonical_url: "https://mysticalresearch.com/topics.html".to_string(),
            site_name: "Angels Project".to_string(),
            site_logo_url: "https://mysticalresearch.com/images/angels-logo.png".to_string(),
            search_action_target: "https://mysticalresearch.com/search?q={search_term_string}"
                .to_string(),
        },
        categories: vec![
            TopicCategory {
                category_icon: "🌙".to_string(),
                category_title: "Lunar Magic".to_string(),
                category_description: "Moon phase rituals".to_string(),
                category_url: "/topics/lunar-magic".to_string(),
            },
            TopicCategory {
                category_icon: "🜁".to_string(),
                category_title: "Elemental Arts".to_string(),
                category_description: "Master the four elements".to_string(),
                category_url: "/topics/elemental-arts".to_string(),
            },
            TopicCategory {
                category_icon: "🌿".to_string(),
                category_title: "Herbalism".to_string(),
                category_description: "The power of plants".to_string(),
                category_url: "/topics/herbalism".to_string(),
            },
        ],
        section: PageType::Topics,
    }
}

pub fn mock_about_home_page_object() -> AboutHomePage {
    AboutHomePage {
        body: AboutBody {
            structure: AboutStructure {
                title: "About Us".to_string(),
                description: "Learn more about our mission and journey.".to_string(),
                about_section_title: "Our Core Values".to_string(),
                items: vec![
                    AboutItem {
                        title: "Value 1".to_string(),
                        description: "Description for value 1.".to_string(),
                        url: "/about/value1".to_string(),
                    },
                    AboutItem {
                        title: "Value 2".to_string(),
                        description: "Description for value 2.".to_string(),
                        url: "/about/value2".to_string(),
                    },
                ],
            },
        },
        schema_markup: AboutHomeSchemaMarkup {
            about_page_title: "About Us - Angels Project".to_string(),
            site_description: "Learn about Angels Project and our mission to empower innovation."
                .to_string(),
            canonical_url: "https://example.com/about.html".to_string(),
            site_name: "Angels Project".to_string(),
            site_logo_url: "/static/images/angels-logo.png".to_string(),
            search_action_target: "https://example.com/search?q={search_term_string}".to_string(),
        },
        header: mock_header_data(),
        section: PageType::About,
    }
}
