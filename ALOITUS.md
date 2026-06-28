<!--
Käytä tätä kunkin session alussa.
-->


# Aloitusprompti

Rust-projekti `esp-zigbee-lib`-kirjaston (v.2.0.1) käyttämiseksi rustista. `esp-idf-sys` (versio 5.5.4), `esp-idf-svc`. Projekti on Cargo workspace, jossa alaprojektit:

|nimi|status|Rust|rooli|
|---|---|---|---|
|`apps`|kääntyy ja linkkautuu|`std`|Ylin sovellustaso. Tavoitteena pitää tämä hyvin yksinkertaisena. <p />Esimerkkinä kaksi `esp_zigbee_lib`:stä portattua sovellusta: `bin/light` ja `bin/switch`.|
|`config`|toimii|`std`, host-puolen käännöstyökalu|Zigbee-noden konfigurointi (vakiot, rooli coordinator/tms.) tulevat TOML-tiedostosta, jonka `config` kääntää koodiksi. Tämä yksinkertaistaa sovellustasoa.|
|`api`|ok|`std`|Varsinainen API-taso.|
|`raw`|Kääntyy, oletettavasti toimii (toimi kirjaston 1.6-versiolla aikoinaan)|`no_std; alloc`|C/Rust -rajapinnan hallinta. Ei nosta abstraktiota, mutta voi paikoitellen helpottaa API-tason tehtäviä lisäämällä ominaisuuksia `bindgen`:n generoimaan raaka-raaka-koodiin. Bindgen-tuotos tulee mukaan `include`:na, mikä mahdollistaa tämän.|

**Arkkitehtuuri**

Sovelluskoodissa on käytössä Embassy. Tämä tarkoittaa, että myös esimerkeissä ajastimien (kohdat, joissa protokollassa odotetaan 1s ja tehdään jotain) sijaan on `async`-viiveet.

Tämä tekee sovellusten kirjoittamisesta ja lukemisesta lineaarista.

`api`-tasolla luodaan Zigbee-kirjastolle oma FreeRTOS-taskinsa (sovellus ei suoraan ole tästä tietoinen). Taskien välillä on `Channel`. Sovellustaso saa nämä Zigbee-viestit asynchronisena streamina (`stream!`).

Toteutuksessa Zigbee-rauta esitetään `&'static Node`:na, ja siihen lisättyinä, profiilikohtaisina trait:eina. Tämä `static`:n käyttö on ennen kaikkea tyylikysymys, sillä C-rajapinta käsittelee kaiken globaaleina funktioina. `Node` tuo koodiin kaivattua "ryhtiä" ja tekee rajapinnat luettavammiksi. Pidetään siitä kiinni.

Sovelluksen ohjatessa etäpään laitetta, se tehdään profiilikohtaisilla `Access...`-traiteilla. Tästä on esimerkki ainakin `apps/bin/switch`-kansiossa.


**Tavoite** 

Saada vastaava koodi kuin C-puolen `color_dimmable_light` ja `color_dimmable_switch` -demot kääntymään ja toimimaan Rust-sovelluksina.


**Tilanne**

Protokollatason `async`-viiveiden vaatimat elinkaarihaasteet on ratkaistu, ja sovellukset kääntyvät onnistuneesti release-profiililla.

Edessä ovat:
- Varsinainen ajokokeilu.
- Valon ja kytkimen toiminnallisuuden viimeistely (ei pelkkä lokitus).
- Seuraavaksi listalla: PoE-virransyöttö ESP32-C6:lle ja LED-ohjaus (Embassy-taskit / PWM).

Akuutti focus: Ajokokeilut ja siirtyminen rautatason toteutuksiin.

Ehdotus: pyydän sinulta apua pienissä ongelmissa, mitä tulee eteen. Saat esittää ehdotuksia.

<!-- #skip
: minulla on ymmärrys Rustin `async`:sta, mutta tämä voi silti olla haastava kohta rakennelmassa.

Tiedän, miten tämä rakennetaan joten kovin paljon aktiivisia ehdotuksia en tässä vaiheessa tarvitse; voit jättää ne pois vastauksista. Kiitos!
-->

## Kehitysalusta (Ubuntu, Multipass, RustRover IDE, google.ai neuvonantajan roolissa)

Teen kehitystyön Multipass VM:n sisältä (Ubuntu 26.04 LTS), hostina on macOS.

- Multipass versio 1.16.3
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

### ESP-IDF käännösketju

`esp-idf-sys`:iä käytetään "native"-moodissa, eli työkalut latautuvat `~/.espressif`-hakemiston alle osana buildia.


## Kommenttien kieli

Koodin kommentit, stringit kirjoitetaan suoraan englanniksi. Muussa vuoropuhelussa käytämme suomea. 


## Palaute

Tämä teksti on `ALOITUS.md`-tiedostossa. 

Jos sinulla on lisäkysyttävää, tuo ne esiin saman tien. :) Samoin *työn aikana* arvostan, jos tuot epävarmoja oletuksia esiin.

