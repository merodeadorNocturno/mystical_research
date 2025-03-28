DEFINE TABLE IF NOT EXISTS blog_article SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS id ON blog_article TYPE record;
DEFINE FIELD IF NOT EXISTS title ON blog_article TYPE option<string>;
DEFINE FIELD IF NOT EXISTS table_of_contents ON blog_article TYPE option<string>;
DEFINE FIELD IF NOT EXISTS summary ON blog_article TYPE option<string>;
DEFINE FIELD IF NOT EXISTS content ON blog_article TYPE option<string>;
DEFINE FIELD IF NOT EXISTS image_urls ON blog_article TYPE option<string>;
DEFINE FIELD IF NOT EXISTS keywords ON blog_article TYPE option<string>;
DEFINE FIELD IF NOT EXISTS author ON blog_article TYPE option<string>;
DEFINE FIELD IF NOT EXISTS page_type ON blog_article TYPE option<string>;
DEFINE FIELD IF NOT EXISTS deleted ON blog_article TYPE option<bool>;
DEFINE FIELD IF NOT EXISTS created_at ON blog_article TYPE option<datetime>;
DEFINE FIELD IF NOT EXISTS updated_at ON blog_article TYPE option<datetime>;
DEFINE FIELD IF NOT EXISTS published_at ON blog_article TYPE option<datetime>;
DEFINE FIELD IF NOT EXISTS slug ON blog_article TYPE option<string>;

DEFINE ANALYZER IF NOT EXISTS my_analyzer TOKENIZERS class FILTERS snowball(english);
DEFINE INDEX IF NOT EXISTS title_index ON TABLE blog_article FIELDS title SEARCH ANALYZER my_analyzer BM25 HIGHLIGHTS;

DEFINE ANALYZER IF NOT EXISTS summary_analyzer TOKENIZERS class FILTERS snowball(english);
DEFINE INDEX IF NOT EXISTS summary_index ON TABLE blog_article FIELDS summary SEARCH ANALYZER summary_analyzer BM25 HIGHLIGHTS;

DEFINE ANALYZER IF NOT EXISTS keywords_analyzer TOKENIZERS class FILTERS snowball(english);
DEFINE INDEX IF NOT EXISTS keywords_index ON TABLE blog_article FIELDS keywords SEARCH ANALYZER keywords_analyzer BM25 HIGHLIGHTS;

DEFINE ANALYZER IF NOT EXISTS table_of_contents_analyzer TOKENIZERS class FILTERS snowball(english);
DEFINE INDEX IF NOT EXISTS table_of_contents_index ON TABLE blog_article FIELDS table_of_contents SEARCH ANALYZER table_of_contents_analyzer BM25 HIGHLIGHTS;

DEFINE ANALYZER IF NOT EXISTS author_analyzer TOKENIZERS class FILTERS snowball(english);
DEFINE INDEX IF NOT EXISTS author_index ON TABLE blog_article FIELDS author SEARCH ANALYZER author_analyzer BM25 HIGHLIGHTS;

DEFINE ANALYZER IF NOT EXISTS content_analyzer TOKENIZERS class FILTERS snowball(english);
DEFINE INDEX IF NOT EXISTS content_index ON TABLE blog_article FIELDS content SEARCH ANALYZER content_analyzer BM25 HIGHLIGHTS;
