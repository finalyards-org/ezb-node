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
