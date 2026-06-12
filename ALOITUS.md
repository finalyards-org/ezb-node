<!--
Käytä tätä kunkin session alussa.
-->


# Aloitusprompti

Rust-projekti `esp-zigbee-lib`-kirjaston (v.2.0.1) käyttämiseksi rustista. `esp-idf-sys` (versio 5.5.4), `esp-idf-svc`. Projekti on Cargo workspace, jossa alaprojektit:

|nimi|status|Rust|rooli|
|---|---|---|---|
|`apps`|"code complete"|`std`|Ylin sovellustaso. Tavoitteena pitää tämä hyvin yksinkertaisena.|
|`config`|toimii|`std`, host-puolen työkalu|Zigbee-noden konfigurointi (vakiot, rooli coordinator/tms.) tulevat TOML-tiedostosta, jonka `config` kääntää koodiksi. Tämä yksinkertaistaa sovellustasoa.|
|`api`|Rust-kääntäjäongelmia esiintyy|`std`|Varsinainen API-taso.|
|`raw`|Kääntyy, oletettavasti toimii (toimi kirjaston 1.6-versiolla aikoinaan)|`no_std; alloc`|C/Rust -rajapinnan hallinta. Ei nosta abstraktiota, mutta voi paikoitellen helpottaa API-tason tehtäviä lisäämällä ominaisuuksia `bindgen`:n generoimaan raaka-raaka-koodiin. Bindgen-tuotos tulee mukaan `include`:na, mikä mahdollistaa tämän.|

Tavoite: saada vastaava koodi kuin C-puolen `color_dimmable_light` (ja sitten `color_dimmable_switch` kääntymään ja toimimaan Rust-sovelluksena).

Tilanne: `raw`, `config` ja `api`-tasot kääntyvät, `apps` ei vielä. Kun kääntyy, edessä ovat:

- kokeilu ESP-IDF 5.5.4 -> 6.0.1 siirtymän mahdollisesta toimivuudesta / raportointi, jos siinä kohtaa ongelmia
- varsinainen ajokokeilu ensin C-vastaesimerkin (switch) kanssa
- valon tekeminen toimivaksi (ei pelkkä lokitus)
- myös switch Rustilla


Akuutti focus: `apps` kääntökuntoon.

Ehdotus: pyydän sinulta apua pienissä ongelmissa, mitä tulee eteen. Tiedän, miten tämä rakennetaan joten kovin paljon aktiivisia ehdotuksia en tässä vaiheessa tarvitse; voit jättää ne pois vastauksista. Kiitos!


## Kehitysalusta (Ubuntu, Multipass, RustRover IDE, google.ai neuvonantajan roolissa)

Teen kehitystyön Multipass VM:n sisältä (Ubuntu 26.04 LTS), hostina on macOS.

- Multipass versio 1.16.2
- Kohderauta: Vain ESP32-C6 (RISC-V, ei muita suunnitteilla)

IDE:nä on RustRover. Lähdekoodit ovat hostin hakemistossa, joka on jaettu VM:n kanssa. `target`-hakemisto on mapattu nopeussyistä Linuxin omaan tiedostojärjestelmään.

Järjestely ei vaadi kehittäjää käyttämään Multipass:ia, vaan minkä tahansa Ubuntu-asennuksen tulee toimia.

### Työkaluketju (`rust-toolchain.toml`)

Koska kohde on RISC-V, käytetään standardia upstream-Rustia ilman espup-työkalua.

```
[toolchain]
channel = "nightly"
components = [ "rust-src", "clippy", "rustfmt" ]
```

Flashaus on etänä ohjattavalla `espflash` (v.4.4.0) -ohjelmalla (VM -> RPi -> ESP32), mutta tämän suhteen ei oikeastaan ole kysyttävää. Homma toimii. :)

## Kommenttien kieli

Koodin kommentit, stringit voit kirjoittaa suoraan englanniksi. Muussa vuoropuhelussa käytämme suomea. 


## Lisäyksiä

- ESP-IDF tuodaan mukaan `esp-idf-sys`:n "native"-moodissa, eli työkalut latautuvat `~/.espressif`-hakemiston alle osana buildia.


## Palaute

Tämä teksti on `ALOITUS.md`-tiedostossa. 

Jos sinulla on lisäkysyttävää, tuo ne esiin saman tien. :) Samoin *työn aikana* arvostan, jos tuot epävarmoja oletuksia esiin.



<!--
## Terminologiasta

*Tähän tulee projektissa käytettävästä termistöstä. Joskus.*

- "state machine" (engl.) on suomeksi "tilakone", ei "valtiokone".
-->

