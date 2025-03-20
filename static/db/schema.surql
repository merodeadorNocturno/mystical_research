DEFINE TABLE IF NOT EXISTS blog_article SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS id ON blog_article TYPE record;
DEFINE FIELD IF NOT EXISTS title ON blog_article TYPE option<string>;
DEFINE FIELD IF NOT EXISTS summary ON blog_article TYPE option<string>;
DEFINE FIELD IF NOT EXISTS keywords ON blog_article TYPE option<string>;
DEFINE FIELD IF NOT EXISTS content ON blog_article TYPE option<string>;
DEFINE FIELD IF NOT EXISTS table_of_contents ON blog_article TYPE option<string>;
DEFINE FIELD IF NOT EXISTS page_type ON blog_article TYPE option<string>;
DEFINE FIELD IF NOT EXISTS deleted ON blog_article TYPE option<bool>;
DEFINE FIELD IF NOT EXISTS author ON blog_article TYPE option<string>;
DEFINE FIELD IF NOT EXISTS image_urls ON blog_article TYPE option<string>;
DEFINE FIELD IF NOT EXISTS created_at ON blog_article TYPE option<datetime>;
DEFINE FIELD IF NOT EXISTS updated_at ON blog_article TYPE option<datetime>;
DEFINE FIELD IF NOT EXISTS published_at ON blog_article TYPE option<datetime>;
