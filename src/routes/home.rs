use askama::Template;
use axum::extract::State;
use axum::response::Html;
use std::time::{SystemTime, UNIX_EPOCH};

use super::render;
use crate::error::AppResult;
use crate::models::{DonationNeed, News};
use crate::site::{Layout, Site};
use crate::state::AppState;

/// The data `templates/pages/home.html` can use. `#[template(path = …)]` ties the struct to the
/// file, and the compiler checks every `{{ … }}` in the template against these fields.
#[derive(Template)]
#[template(path = "pages/home.html")]
struct HomePage {
    layout: Layout,
    latest_news: Vec<News>,
    donation_needs: Vec<DonationNeed>,
    verse: &'static Verse,
    pillars: &'static [Pillar],
    contacts: &'static [Contact],
}

/// `GET /`
///
/// Handler parameters are *extractors*: axum looks at each one's type and fills it from the
/// request. `State(state)` destructures the wrapper so `state` is the `AppState` itself.
pub async fn show(State(state): State<AppState>, site: Site) -> AppResult<Html<String>> {
    // `try_join!` runs both queries at the same time and waits for both; if either fails, `?`
    // returns its error. In PHP they would run one after the other.
    let (latest_news, donation_needs) = tokio::try_join!(
        News::latest(&state.db, 3),
        DonationNeed::latest_active(&state.db, 6),
    )?;

    let layout = Layout::new(&site, "Почетна").meta(
        "Почетна",
        "Фондацијата „Адријан Живков\" е посветена на поддршка на семејствата, донации за деца и хуманитарни активности во заедницата.",
    );

    render(&HomePage {
        layout,
        latest_news,
        donation_needs,
        verse: random_verse(),
        pillars: PILLARS,
        contacts: CONTACTS,
    })
}

/// A different verse on each visit. The Vue page picked one in the browser; here the server does,
/// using the clock as a cheap source of randomness so no extra crate is needed.
fn random_verse() -> &'static Verse {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|elapsed| elapsed.subsec_nanos())
        .unwrap_or(0);

    &VERSES[nanos as usize % VERSES.len()]
}

pub struct Verse {
    pub reference: &'static str,
    pub text: &'static str,
}

pub struct Pillar {
    pub text: &'static str,
    /// An SVG path drawn in the card's icon.
    pub icon: &'static str,
}

pub struct Contact {
    pub label: &'static str,
    pub value: &'static str,
    pub href: &'static str,
    pub external: bool,
    pub icon: &'static str,
}

// `&'static str` is text compiled into the program itself: it lives for the program's whole run
// (the `'static` *lifetime*), so it can be shared freely without copying.
const VERSES: &[Verse] = &[
    Verse {
        reference: "Јован 15:13",
        text: "Никој нема поголема љубов од оваа: да го положи сопствениот живот за своите пријатели.",
    },
    Verse {
        reference: "Матеј 25:40",
        text: "Вистина ви велам, доколку му направивте на еден од овие Мои најмали браќа, Мене Ми направивте.",
    },
    Verse {
        reference: "Галатјани 6:2",
        text: "Носете ги бремињата еден на друг, и со тоа ќе го исполните законот Христов.",
    },
    Verse {
        reference: "1 Коринтјани 13:13",
        text: "А сега остануваат овие три: верата, надежта и љубовта, но љубовта е најголема од нив.",
    },
    Verse {
        reference: "Римјаните 8:38-39",
        text: "Зашто сум уверен дека ни смртта, ни животот, ни ангелите, ни властите, ни сегашноста, ни иднината, ни силите, ни височината, ни длабочината, ниту некое друго создание, ќе може да нѐ оддели од Божјата љубов, која е во Христа Исуса, нашиот Господ.",
    },
    Verse {
        reference: "Псалм 23:4",
        text: "Ако тргнам и низ долината на смртната сенка, нема да се уплашам од злото, зашто Ти си со мене. Твојот жезал и Твојата палка ме утешуваат.",
    },
    Verse {
        reference: "Исаија 41:10",
        text: "Не бој се, зашто Јас Сум со тебе; не обрнувај се исплашено, зашто Јас Сум твој Бог. Јас те крепам и ти помагам, те потпирам со победничка десница.",
    },
    Verse {
        reference: "Марко 10:45",
        text: "Зашто и Човечкиот Син не дојде да Му служат, туку да служи и да ја даде Својата душа за откуп на мнозина.",
    },
    Verse {
        reference: "Притчи 17:17",
        text: "Пријателот љуби во секое време, а во неволја станува и брат.",
    },
    Verse {
        reference: "Откровение 21:4",
        text: "И ќе ја избрише секоја солза од нивните очи, и смрт нема да има веќе, ни тага, ни плач, ниту болка, зашто првото помина.",
    },
];

const PILLARS: &[Pillar] = &[
    Pillar {
        text: "Мисијата на Фондацијата „Адријан Живков\" е да создаде трајна поддршка за поединци и семејства кои се соочуваат со тешки животни околности. Преку хуманитарни активности, директна помош и општествено ангажирање, се стремиме да бидеме извор на надеж.",
        icon: "M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z",
    },
    Pillar {
        text: "Водени од вредностите на солидарност, емпатија и несебичност кои Адријан ги живееше, нашата цел е да изградиме заедница која дејствува со одговорност и грижа.",
        icon: "M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0z",
    },
    Pillar {
        text: "Нашата мисија е насочена кон создавање трајно позитивно влијание, преку конкретни дела кои оставаат значење денес и надеж за иднината.",
        icon: "M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z",
    },
];

const CONTACTS: &[Contact] = &[
    Contact {
        label: "Е-пошта",
        value: "adrijanfoundation@gmail.com",
        href: "mailto:adrijanfoundation@gmail.com",
        external: false,
        icon: "M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z",
    },
    Contact {
        label: "Телефон",
        value: "+389 77720475",
        href: "tel:+38977720475",
        external: false,
        icon: "M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z",
    },
    Contact {
        label: "Инстаграм",
        value: "@adrijanzivkov",
        href: "https://www.instagram.com/adrijanzivkov/",
        external: true,
        icon: "M16 11.37A4 4 0 1112.63 8 4 4 0 0116 11.37zM17.5 6.5h.01M7 2h10a5 5 0 015 5v10a5 5 0 01-5 5H7a5 5 0 01-5-5V7a5 5 0 015-5z",
    },
];
