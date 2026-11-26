//! Splitting a list into pages, like Laravel's `->paginate()` and its page links.

/// One page of results, plus what's needed to draw the page links under them.
///
/// `<T>` makes it *generic*: `Paginator<News>` and `Paginator<DonationNeed>` share this code,
/// and the compiler checks each one separately, like a typed version of a PHP collection.
pub struct Paginator<T> {
    pub items: Vec<T>,
    pub current_page: u32,
    pub per_page: u32,
    pub total: u32,
    /// The page's own path, e.g. `/news`, to build `?page=2` links.
    path: String,
}

/// One button in the page links: a number, `...`, or previous/next.
pub struct PageLink {
    /// `None` for `...` and for previous/next when there's nowhere to go.
    pub url: Option<String>,
    pub label: String,
    pub active: bool,
}

impl<T> Paginator<T> {
    pub fn new(items: Vec<T>, total: u32, page: Page, path: &str) -> Self {
        Self {
            items,
            current_page: page.number,
            per_page: page.size,
            total,
            path: path.to_string(),
        }
    }

    pub fn last_page(&self) -> u32 {
        // `div_ceil` rounds up: 25 items at 12 per page make 3 pages. Never fewer than 1 page.
        self.total.div_ceil(self.per_page).max(1)
    }

    fn url(&self, page: u32) -> String {
        format!("{}?page={page}", self.path)
    }

    /// The page links, with `...` gaps on long lists, chosen exactly as Laravel's `UrlWindow` does:
    /// all pages when there are fewer than 14, otherwise the first two, the last two and three
    /// on each side of the current page.
    pub fn links(&self) -> Vec<PageLink> {
        let last = self.last_page();
        let current = self.current_page;
        const ON_EACH_SIDE: u32 = 3;
        let window = ON_EACH_SIDE + 4;

        // A `Vec` of ranges; `None` marks a `...` gap between them.
        let sections: Vec<Option<std::ops::RangeInclusive<u32>>> = if last < ON_EACH_SIDE * 2 + 8 {
            vec![Some(1..=last)]
        } else if current <= window {
            vec![Some(1..=window + ON_EACH_SIDE), None, Some(last - 1..=last)]
        } else if current > last - window {
            vec![
                Some(1..=2),
                None,
                Some(last - (window + ON_EACH_SIDE - 1)..=last),
            ]
        } else {
            vec![
                Some(1..=2),
                None,
                Some(current - ON_EACH_SIDE..=current + ON_EACH_SIDE),
                None,
                Some(last - 1..=last),
            ]
        };

        let mut links = vec![PageLink {
            url: (current > 1).then(|| self.url(current - 1)),
            label: "« Претходно".to_string(),
            active: false,
        }];

        for section in sections {
            match section {
                Some(range) => links.extend(range.map(|page| PageLink {
                    url: Some(self.url(page)),
                    label: page.to_string(),
                    active: page == current,
                })),
                None => links.push(PageLink {
                    url: None,
                    label: "...".to_string(),
                    active: false,
                }),
            }
        }

        links.push(PageLink {
            url: (current < last).then(|| self.url(current + 1)),
            label: "Следно »".to_string(),
            active: false,
        });

        links
    }
}

/// Which page was asked for, and how many items go on it.
#[derive(Clone, Copy)]
pub struct Page {
    pub number: u32,
    pub size: u32,
}

impl Page {
    /// Reads `?page=` leniently, as Laravel does: anything missing, not a number or below 1 is page 1.
    pub fn from_query(query: &PageQuery, size: u32) -> Self {
        let number = query
            .page
            .as_deref()
            .and_then(|value| value.parse::<u32>().ok())
            .filter(|number| *number >= 1)
            .unwrap_or(1);

        Self { number, size }
    }

    /// How many rows to skip, for SQL's `OFFSET`.
    pub fn offset(&self) -> u32 {
        (self.number - 1) * self.size
    }
}

/// The query string, read by axum's `Query` extractor. `#[derive(Deserialize)]` generates the
/// parsing code at compile time. `page` is kept as text so a bad value like `?page=abc` falls
/// back to page 1 instead of failing the request.
#[derive(serde::Deserialize)]
pub struct PageQuery {
    pub page: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn labels(total: u32, current: u32) -> Vec<String> {
        let page = Page {
            number: current,
            size: 10,
        };
        Paginator::<()>::new(vec![], total, page, "/news")
            .links()
            .into_iter()
            .map(|link| link.label)
            .collect()
    }

    #[test]
    fn shows_every_page_on_short_lists() {
        assert_eq!(labels(30, 1), ["« Претходно", "1", "2", "3", "Следно »"]);
    }

    #[test]
    fn matches_laravel_window_in_the_middle_of_a_long_list() {
        // The same 400 items, 10 per page, page 20 that Laravel was checked against.
        assert_eq!(
            labels(400, 20),
            [
                "« Претходно",
                "1",
                "2",
                "...",
                "17",
                "18",
                "19",
                "20",
                "21",
                "22",
                "23",
                "...",
                "39",
                "40",
                "Следно »"
            ]
        );
    }

    #[test]
    fn disables_previous_on_the_first_page_and_next_on_the_last() {
        let first = Paginator::<()>::new(
            vec![],
            30,
            Page {
                number: 1,
                size: 10,
            },
            "/news",
        )
        .links();
        assert!(first[0].url.is_none());
        assert_eq!(first.last().unwrap().url.as_deref(), Some("/news?page=2"));

        let last = Paginator::<()>::new(
            vec![],
            30,
            Page {
                number: 3,
                size: 10,
            },
            "/news",
        )
        .links();
        assert!(last.last().unwrap().url.is_none());
    }

    #[test]
    fn reads_bad_page_numbers_as_page_one() {
        for value in ["abc", "0", "-2", ""] {
            let query = PageQuery {
                page: Some(value.to_string()),
            };
            assert_eq!(Page::from_query(&query, 12).number, 1);
        }
    }
}
