Things to do with `google.ai`

---

## Markdown session aloitukseen

Tee lyhyt kooste, mistä projektissa on kyse. Sen voi antaa sitten liitteenä uuden session alkaessa.


## Zigbee Network Logic & Factory New Status

- [ ] **Elinkaaren hallinta**: Hyödynnä `esp_zb_bdb_is_factory_new()` Routerin alustuksessa.
    - *Miksi:* Jos laite on jo verkossa (ei Factory New), sen ei pitäisi aloittaa "Network Steeringiä" alusta, vaan tehdä "Rejoin".
    - *Logiikka:* 
        - `true`  -> Vilkuta LEDiä ja odota `start_commissioning`-kutsua.
        - `false` -> Jatka suoraan normaaliin operointiin tai yritä yhteyden palautusta.
    - *Status:* Nykyisin seuraa suoraan C-esimerkin lineaarista kulkua.

>Askon kommentti: 
> siirretään `light`-esimerkistä paljon yksityiskohtia `Router`:in sisään, koettaen saada light yksinkertaiseksi. `light.light`.


## **Asynktoninen** ZCL-eventteihin vastaaminen

```
Tämä toimii synkronisessa callbackin käsittelyssä. 

Minun tapauksessani Zigbee-taskin puolella saatu callback konvertoi eventin Rustille sopivaan muotoon, laittaa sen `Channel`:iin ja palaa.

Se, mitä toisessa (sovellustaskissa) halutaan kyseiselle eventille paluuarvoksi ei ole vielä tiedossa.Tämä muuttaa tilannetta kriittisesti. Koska siirrät viestin Channelin kautta toiselle taskille ja callback-funktio palaa välittömästi, C-puoli vapauttaa tai uusiokäyttää kyseisen muistiosoitteen sekunnin murto-osassa.Tämän vuoksi aiemmin suunniteltu out.result-kenttään kirjoittaminen toisesta taskista on mahdotonta. Kun toinen taski herää käsittelemään viestiä, alkuperäinen callback on jo päättynyt, ja pino on lukenut sen hetkisen (tyhjän) arvon.Jos haluat lähettää sovellustaskista vastauksen (kuten virhekoodin tai statuksen) takaisin Zigbee-verkolle, sinulla on kaksi vaihtoehtoa arkkitehtuurille:Vaihtoehto 1: Lähetä ZCL-vastaus asynkronisesti käsin (Suositeltu)Tämä on yleisin malli asynkronisissa Rust-arkkitehtuureissa.Annat callbackin päättyä normaalisti (se palauttaa oletuksena Success-tilan pinolle).Sovellustaski käsittelee viestin omaan tahtiinsa.Kun sovellustaski tietää lopputuloksen, se kutsuu esp_zigbee_lib:n funktiota (kuten esp_zb_zcl_send_default_reply tai vastaavaa komentokohtaista lähetysfunktiota) lähettääkseen virheen tai onnistumisen verkon yli takaisin kaverille.Tässä mallissa sinun on kopioitava callbackissa talteen viestin lähdeosoite, endpoint, cluster ID ja sequence number, jotta sovellustaski osaa osoittaa vastauksen oikealle vastaanottajalle.Vaihtoehto 2: Pysäytä callback odottamaan (Synkronointi kanavalla)Jos ESP-Zigbee-pino ehdottomasti vaatii, että vastaus täytyy antaa juuri siinä callbackissa (eikä pino tue erillistä asynkronista vastausfunktiota), sinun täytyy pysäyttää Zigbee-taski odottamaan sovellustaskun päätöstä.Voit toteuttaa tämän käyttämällä "oneshot"-kanavaa, joka lähetetään eventin mukana:rust// 1. Määrittele viesti, joka lähtee sovellustaskille
struct ZigbeeEventMessage {
    data: MyRustEnum,
    // Kanava, jolla sovellustaski vastaa takaisin callbackille
    response_tx: oneshot::Sender<esp_zb_zcl_status_t>, 
}

// 2. Callbackissa (Zigbee-taski)
fn on_zcl_event_callback(vp: *mut ::core::ffi::c_void) {
    let msg_ref = unsafe { (vp as *mut MyCStruct).as_mut().unwrap() };
    
    // Luodaan paluukanava
    let (tx, rx) = oneshot::channel();
    
    let event = ZigbeeEventMessage {
        data: convert_to_rust_enum(msg_ref),
        response_tx: tx,
    };
    
    // Lähetetään sovellukselle
    app_channel.send(event).ok();
    
    // Blokataan Zigbee-taski siihen asti, kunnes sovellus vastaa
    if let Ok(status) = rx.recv_blocking() {
        // Kirjoitetaan sovelluksen päättämä status suoraan C-structiin
        msg_ref.out.result = status;
    }
}
Käytä koodia harkiten.Huomautus: Vaihtoehto 2 pysäyttää Zigbee-protokollapinnan ajon odotuksen ajaksi. Jos sovellustaskilla kestää liian kauan (esim. se tekee hitaita I/O-operaatioita), Zigbee-pino voi jumittua tai pudota verkosta. Siksi Vaihtoehto 1 on aina turvallisempi, jos kirjasto vain sallii erillisen vastausviestin lähettämisen.Kumpaa mallia ajattelit lähteä kokeilemaan, tai tukeeko kyseinen ZCL-komento suoraan asynkronista vastausta (send_default_reply)?Tekoälyn antamat vastaukset voivat sisältää virheitä. Lue lisääSanoit: Vaihtoehto 1. Eli Zigbeessä on ok, vaikka samalle viestille tulisi pari erilaista vastausta?Nyrkkisääntönä on, että Zigbee-verkko sietää tämän hyvin, mutta sinun on oltava tarkkana sen kanssa, mitä esp_zigbee_lib tekee taustalla.Jos käytät Vaihtoehtoa 1, verkon yli saattaa lähteä kaksi vastausta:Automaattinen kuittaus: Kun callback päättyy, ESP:n Zigbee-pino luulee kaiken olevan kunnossa ja lähettää automaattisesti Default Response (Success) -viestin.Sinun sovelluksesi vastaus: Myöhemmin sovellustaskisi lähettää oman vastauksensa (esim. virhekoodin tai dataa).Miten Zigbee-laitteet reagoivat kahteen vastaukseen?Zigbee-spesifikaatio käsittelee tämän tilanteen seuraavasti riippuen siitä, millainen komento oli kyseessä:Jos kyseessä on komento, joka vaatii datavastauksen (esim. Read Attributes):Laitteet odottavat saavansa Read Attributes Response -viestin. Jos ESP-pino lähettää ensin tyhjän Default Response (Success) -kuittauksen callbackin päätyttyä, ja sinä lähetät heti perään oikean datavastauksen, pyynnön lähettänyt laite saattaa hämmentyä tai jättää jälkimmäisen datan huomiotta, koska se katsoo transaktion jo suljetuksi.Jos kyseessä on toimintakomento (esim. On, Off, Toggle):Nämä komennot vaativat vastineeksi vain pelkän kuittauksen (Default Response). Jos laite saa ensin Success-kuittauksen ja hetken päästä sovellukseltasi Failure-kuittauksen, se yleensä luottaa ensimmäiseen saapuneeseen viestiin. Kaksinkertainen vastaus kuluttaa myös turhaan verkon kaistanleveyttä.Miten estät ESP-pinoa lähettämästä automaattista kuittausta?Jotta vältyt tuplavastauksilta, sinun täytyy kertoa ESP:n protokollapinolle jo callback-funktion sisällä, ettei se saa lähettää automaattista kuittausta.Tämä tehdään asettamalla callbackin paluuarvoksi (tai event-rakenteen kenttään) erityinen status, joka tarkoittaa "Sovellus hoitaa vastaamisen itse myöhemmin".Etsi esp_zigbee_lib:n otsikkotiedostoista (tai bindgen-generoidusta koodista) enum-arvo, joka vastaa statusta ESP_ZB_ZCL_STATUS_CMD_ASYNC tai ZB_ZCL_STATUS_SUCCESS_ASYNC (tarkka nimi riippuu ESP-IDF:n versiosta).Kun asetat tämän arvon callbackissa, pino pysyy hiljaa:rust// Callback-funktiossa (Zigbee-taski)
fn on_zcl_event_callback(vp: *mut ::core::ffi::c_void) {
    let msg_ref = unsafe { (vp as *mut MyCStruct).as_mut().unwrap() };
    
    // 1. Kopioidaan tiedot kanavaan sovellustaskille...
    
    // 2. Estetään automaattinen kuittaus kertomalla pinolle, 
    // että sovellus vastaa tähän asynkronisesti myöhemmin.
    msg_ref.out.result = esp_zigbee_sys::esp_zb_zcl_status_t_ESP_ZB_ZCL_STATUS_CMD_ASYNC;
}
Käytä koodia harkiten.Tämän ansiosta ESP-pino ei lähetä mitään, ja sovellustaskillasi on täysi rauha ja oikeus lähettää se ainoa ja oikea vastaus verkon yli silloin, kun se on valmis.
```

## Milloin palauttaa `false` C-callbackista

```
C:n callback palauttaa `bool`. Luuletko (tiedätkö), mitä tapahtuu jos palautan sille `false`? Heidän dokut eivät asiasta mielestäni mainitse. 

```
extern "C" fn app_signal_handler(p_app_signal: *const ezb_app_signal_t) -> bool ```ESP-Zigbee-SDK:ssa (joka perustuu taustalla ZBOSS-Zigbee-pinoon) tällä bool-palautusarvolla on erittäin kriittinen ja tarkka merkitys.Kun app_signal_handler palauttaa arvon:true: Kerrot Zigbee-pinolle, että sinun sovelluksesi käsitteli (tai aikoo käsitellä) tämän signaalin kokonaan. Pino päättelee, että signaali on hoidettu, eikä se tee sille enää mitään automaattisia tausta-toimenpiteitä.false: Ilmoitat pinolle, että sovelluksesi hylkäsi tai jätti signaalin huomiotta. Tällöin Zigbee-pino suorittaa kyseiselle signaalille sen oman sisäänrakennetun oletustoiminnon (default action).Mitä false:n palauttaminen aiheuttaa käytännössä?Se riippuu täysin siitä, mikä p_app_signal-rakenteen sisältämä signaalityyppi (kuten ESP_ZB_ZDO_SIGNAL_SKIP_STARTUP, ESP_ZB_BDB_SIGNAL_DEVICE_REBOOT tai ESP_ZB_BDB_SIGNAL_STEERING) on kyseessä:Verkkoon liittyminen ja paritus (Commissioning / Steering):Jos saat signaalin, joka kertoo laitteen liittyneen verkkoon onnistuneesti, ja palautat false, pino saattaa ajaa oman oletuslogiikkansa – esimerkiksi tallentaa verkkoparametrit NVRAM-muistiin ja sulkea paritusikkunan automaattisesti. Jos taas palautat true, pino olettaa, että sovelluksesi hoitaa parituksen loppuvaiheet (kuten endpointien esittelyt) itse käsin.Virhetilanteet (kuten verkon hukkuminen):Jos pino lähettää signaalin siitä, että yhteys reitittimeen katkesi, ja palautat false, pinon oletustoiminto voi olla käynnistää automaattinen uudelleenliittymisprosessi (rejoin). Jos palautat true, pino pysyy passiivisena, koska se luulee sovelluksesi hoitavan verkon etsinnän omalla logiikallaan.Miten sinun kannattaa toimia Rust-koodissasi?Koska arkkitehtuurissasi kopioit viestin kanavaan ja palaat callbackista heti, paras ja turvallisin nyrkkisääntö on:Palauta false kaikille niille signaaleille, joita sovelluksesi ei tunmista tai joille et ole vielä kirjoittanut käsittelyloogikaa. Näin varmistat, että ESP-Zigbee-pino hoitaa perusohjauksen, eikä laite esimerkiksi jää jumiin verkon käynnistyksessä.Palauta true vain silloin, kun tiedät tismalleen mitä teet ja olet kopioinut signaalin onnistuneesti eteenpäin sovellustaskille, ja haluat nimenomaan estää C-puolen oletuskäyttäytymisen.Peruskäytössä (kuten laitteen käynnistyessä ja perusohjaussignaaleissa) false on usein turvallisempi oletusarvo, koska se antaa ESP-IDF:n oman Zigbee-pohja-automatiikan tehdä työt puolestasi taustalla.
```

Tätä täytyy vielä katsoa uudelleen.

Nykyinen toiminta C-esimerkin mukaista, mutta se ei sovellu varsinaisiin Rust-koodeihin. Meillä pitäisi olla jokin synkroninen tapa sovelluksen määritellä (TOML, ehkä??), mitkä asiat se hoitaa itse.

