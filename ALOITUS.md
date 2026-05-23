<!--
Käytä tätä kunkin session alussa.
-->


# Aloitusprompti

Rust-projekti `esp-zigbee-lib`-kirjaston käyttämiseksi rustista. `esp-idf-sys`, `esp-idf-svc`. Projekti on Cargo workspace, jossa alaprojektit:

- apps
- api
- raw
- config

Tavoite: saada vastaava koodi kuin C-puolen `color_dimmable_light` (ja sitten `color_dimmable_switch` kääntymään ja toimimaan Rust-sovelluksena).

Tilanne: rakenne pääosin kunnossa. Bindgen toimii.

Akuutti focus: kirjastosta tuli uusi 2.0.1-versio. Otan sen käyttöön ja jatkan vielä kesken olevia transitioita.

Ehdotus: pyydän sinulta apua pienissä ongelmissa, mitä tulee eteen. Tiedän, miten tämä rakennetaan joten kovin paljon aktiivisia ehdotuksia en tässä vaiheessa tarvitse; voit jättää ne pois vastauksista. Kiitos!

## Kehitysalusta

Teen kehitystyön Multipass VM:n sisältä (Ubuntu 26.04 LTS), hostina on macOS.

- Multipass versio 1.16.2

IDE:nä on RustRover. Lähdekoodit ovat hostin hakemistossa, joka on jaettu VM:n kanssa. `target`-hakemisto on mapattu nopeussyistä Linuxin omaan tiedostojärjestelmään.

Järjestely ei vaadi kehittäjää käyttämään Multipass:ia, vaan minkä tahansa Ubuntu-asennuksen tulee toimia.


## Terminologiasta

- "state machine" (engl.) on suomeksi "tilakone", ei "valtiokone".
