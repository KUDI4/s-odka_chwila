use dioxus::prelude::*;

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let przepis_kalkulator = use_signal(|| String::new());
    let co_liczymy = use_signal(|| "ml na g".to_string());
    let mut szukaj = use_signal(String::new);
    let mut czy_pokazac_ksiazke = use_signal(|| true); 

    let przepisy = vec![
        ("Naleśniki", "2 jajka, 250 ml mleka, 150 g mąki", "Wymieszaj składniki i usmaż na patelni."),
        ("Gofry", "2 jajka, 250 ml mleka, 200 g mąki, 50 g masła", "Wymieszaj składniki i upiecz w gofrownicy."),
        ("Pizza", "250 g mąki, 150 ml wody, drożdże, sos, ser", "Zrób ciasto, dodaj składniki i upiecz."),
        ("Spaghetti", "200 g makaronu, mięso mielone, sos pomidorowy", "Ugotuj makaron i przygotuj sos."),
        ("Omlet", "2 jajka, 2 łyżki mleka, sól", "Wymieszaj składniki i usmaż na patelni."),
        ("Pancakes", "200 g mąki, jajko, 200 ml mleka, cukier", "Wymieszaj składniki i usmaż małe placuszki."),
        ("Brownie", "100 g czekolady, 100 g masła, 2 jajka, 100 g cukru, 70 g mąki", "Wymieszaj składniki i piecz około 20 minut.")
    ];

    rsx! {
        style {
            {
                "
                            body {
                                font-family: Arial, sans-serif;
                                background: #f2f2f2;
                                margin: 0;
                            }

                            .kontener {
                                max-width: 800px;
                                margin: 40px auto;
                                padding: 20px;
                            }

                            .sekcja {
                                background: white;
                                padding: 25px;
                                margin-bottom: 30px;
                                border-radius: 12px;
                                box-shadow: 0 4px 10px rgba(0,0,0,0.1);
                            }

                            h1 {
                                text-align: center;
                                color: #333;
                                margin-bottom: 30px;
                            }

                            .naglowek-zwijany {
                                display: flex;
                                justify-content: space-between;
                                align-items: center;
                                cursor: pointer;
                                border-bottom: 2px solid #eee;
                                padding-bottom: 10px;
                                user-select: none; /* Zapobiega zaznaczaniu tekstu przy klikaniu */
                            }

                            .naglowek-zwijany:hover {
                                opacity: 0.8;
                            }

                            h2 {
                                color: #27864b;
                                margin: 0;
                                border: none;
                                padding: 0;
                            }

                            h3 {
                                color: #333;
                                margin-top: 0;
                            }

                            input, select, textarea {
                                width: 100%;
                                padding: 12px;
                                margin-bottom: 15px;
                                box-sizing: border-box;
                                border: 1px solid #ccc;
                                border-radius: 8px;
                                font-size: 16px;
                                font-family: Arial, sans-serif;
                            }

                            select {
                                cursor: pointer;
                                background-color: #f9f9f9;
                            }

                            .przepis-karta {
                                background: #fafafa;
                                padding: 15px;
                                margin-bottom: 15px;
                                border-radius: 8px;
                                border: 1px solid #eee;
                            }
                        "
            }
        }

        div { class: "kontener",
            h1 { "Słodka minutka" }

            div { class: "sekcja",
                div {
                    class: "naglowek-zwijany",
                    onclick: move |_| czy_pokazac_ksiazke.set(!czy_pokazac_ksiazke()),
                    h2 { "Książka kucharska" }
                    span { font_size: "20px",
                        if czy_pokazac_ksiazke() {
                            "▲"
                        } else {
                            "▼"
                        }
                    }
                }

                if czy_pokazac_ksiazke() {
                    div { margin_top: "20px",
                        input {
                            r#type: "text",
                            placeholder: "Szukaj przepisu...",
                            value: "{szukaj}",
                            oninput: move |event| szukaj.set(event.value()),
                        }

                        for przepis in przepisy.iter().filter(|p| p.0.to_lowercase().contains(&szukaj().to_lowercase())) {
                            div { class: "przepis-karta",
                                h3 { "{przepis.0}" }
                                p {
                                    strong { "Składniki: " }
                                    "{przepis.1}"
                                }
                                p {
                                    strong { "Przygotowanie: " }
                                    "{przepis.2}"
                                }
                            }
                        }

                        if przepisy.iter().all(|p| !p.0.to_lowercase().contains(&szukaj().to_lowercase())) {
                            p { "Nie znaleziono przepisu." }
                        }
                    }
                }
            }

            div { class: "sekcja",
                div {
                    class: "naglowek-zwijany",
                    style: "cursor: default; border-bottom: 2px solid #eee;",
                    h2 { "Kalkulator Kuchenny" }
                }
                div { margin_top: "15px",
                    p {
                        "Skopiuj składniki z przepisu powyżej i wklej je poniżej, aby przeliczyć jednostki."
                    }

                    Przepis { przepis: przepis_kalkulator }
                    Co_liczymy { co_liczymy }
                    Przepis_zmieniony { co_liczymy, przepis: przepis_kalkulator }
                }
            }
        }
    }
}

#[component]
fn Przepis(mut przepis: Signal<String>) -> Element {
    let ilosc_linii = przepis().lines().count().max(3);
    rsx! {
        div {
            textarea {
                value: "{przepis}",
                oninput: move |evt| przepis.set(evt.value()),
                rows: "{ilosc_linii}",
                resize: "vertical",
                placeholder: "Wklej przepis...",
            }
        }
    }
}

#[component]
fn Co_liczymy(mut co_liczymy: Signal<String>) -> Element {
    rsx! {
        div {
            select {
                onchange: move |evt| co_liczymy.set(evt.value()),
                value: "{co_liczymy}",

                option { value: "ml na g", "Mililitry na gramy" }
                option { value: "g na ml", "Gramy na mililitry" }
                option { value: "szklanki na g", "Szklanki na gramy" }
                option { value: "g na szklanki", "Gramy na szklanki" }
                option { value: "lyzki na g", "Łyżki na gramy" }
                option { value: "lyzeczki na g", "Łyżeczki na gramy" }
                option { value: "g na lyzki", "Gramy na łyżki" }
            }
        }
    }
}

#[component]
fn Przepis_zmieniony(co_liczymy: Signal<String>, przepis: Signal<String>) -> Element {
    let tekst_wyniku = przelicznik(&co_liczymy(), &przepis());
    let ilosc_linii = tekst_wyniku.lines().count().max(3);

    rsx!{
        div {
            textarea {
                value: "{tekst_wyniku}",
                readonly: "true",
                rows: "{ilosc_linii}",
                resize: "vertical",
                placeholder: "Tutaj pojawi się przepis po przeliczeniu",
                background_color: "#f8f9fa",
            }
        }
    }
}

fn przelicznik(co_liczymy: &str, przepis: &str) -> String {
    let mut ostateczny_wynik = String::new();

    for linia in przepis.lines() {
        let slowa: Vec<&str> = linia.split_whitespace().collect();
        let mut nowa_linia: Vec<String> = Vec::new(); 
        let mut i = 0;

        while i < slowa.len() {
            let slowo = slowa[i];
            let parsowane = slowo.replace(",", ".").parse::<f64>();

            if let Ok(liczba) = parsowane {
                if i + 1 < slowa.len() {
                    let jednostka = slowa[i + 1].to_lowercase();
                    let skladnik = if i + 2 < slowa.len() { slowa[i + 2] } else { "" };
                    let gestosc = pobierz_gestosc(skladnik); 
                    let mut przeliczono = false;

                    match co_liczymy {
                        "ml na g" => {
                            if jednostka == "ml" {
                                let wynik = liczba * gestosc;
                                nowa_linia.push(format!("{:.0} g", wynik));
                                przeliczono = true;
                            }
                        }
                        "g na ml" => {
                            if jednostka.starts_with("g") || jednostka.starts_with("gram") {
                                let wynik = liczba / gestosc;
                                nowa_linia.push(format!("{:.0} ml", wynik));
                                przeliczono = true;
                            }
                        }
                        "szklanki na g" => {
                            if jednostka.starts_with("szklan") {
                                let wynik = liczba * 250.0 * gestosc; 
                                nowa_linia.push(format!("{:.0} g", wynik));
                                przeliczono = true;
                            }
                        }
                        "g na szklanki" => {
                            if jednostka.starts_with("g") || jednostka.starts_with("gram") {
                                let wynik = (liczba / gestosc) / 250.0;
                                nowa_linia.push(format!("{:.1} szklanki", wynik));
                                przeliczono = true;
                            }
                        }
                        "lyzki na g" => {
                            if jednostka.starts_with("łyżk") || jednostka.starts_with("lyzk") {
                                let wynik = liczba * 15.0 * gestosc; 
                                nowa_linia.push(format!("{:.0} g", wynik));
                                przeliczono = true;
                            }
                        }
                        "lyzeczki na g" => {
                            if jednostka.starts_with("łyżecz") || jednostka.starts_with("lyzecz") {
                                let wynik = liczba * 5.0 * gestosc; 
                                nowa_linia.push(format!("{:.0} g", wynik));
                                przeliczono = true;
                            }
                        }
                        "g na lyzki" => {
                            if jednostka.starts_with("g") || jednostka.starts_with("gram") {
                                let wynik = (liczba / gestosc) / 15.0;
                                nowa_linia.push(format!("{:.1} łyżki", wynik));
                                przeliczono = true;
                            }
                        }
                        _ => {}
                    }

                    if przeliczono {
                        i += 2;
                        continue; 
                    }
                }
            }
            
            nowa_linia.push(slowo.to_string());
            i += 1;
        }
        
        ostateczny_wynik.push_str(&nowa_linia.join(" "));
        ostateczny_wynik.push('\n');
    }

    ostateczny_wynik
}

fn pobierz_gestosc(skladnik: &str) -> f64 {
    let s = skladnik.to_lowercase();
    
    if s.contains("olej") || s.contains("oliw") {
        0.92
    } else if s.contains("mąk") || s.contains("mak") {
        0.60 
    } else if s.contains("cukier") || s.contains("cukru") {
        0.85
    } else if s.contains("masł") || s.contains("masl") {
        0.95
    } else if s.contains("miód") || s.contains("miod") {
        1.42
    } else if s.contains("kaka") { 
        0.50
    } else {
        1.0 
    }
}