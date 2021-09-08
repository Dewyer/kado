# Advent of code but for SCH QPA

na ez egy jó projekt lesz
- Fő célok:
  - Emailek gyűjtése
  - Okosak kiszűrése
  - Minden napra egy programming challange
- How?
  - Google signing
  - DE
    - Az api elég könnyen kiegészíthető ujabb providerekkel
    - Github
    - Reddit
  - API based task solving
  - admin UI ? we will figure it out

### Featureok

- Google Sign on
- Teamekbe összejövés pontok miatt!
  - Solves per team give diminishing returns
- Individual rankings too ?
  - User can choose to be ranked or not ?
  - Register flow gives username
  - protected with captcha
- Tasks screen
  - Gives list of tasks
- Can check out tasks one by one
  - Markdown description maybe
  - and details FOR API
- COMM meg a feladat leadása
  - APIN keresztül történik
  - Ez extra challange
  - Mi leadjuk a bementet
  - A feladat leírása megmondja mi kell legyena  kimenet
  - Visszaküldik a kimenetet
  - Mégis!! beolvassuk a kódot de nem futtatjuk le, csak plágium checking miatt.
    - Ha ugye valaki SUSS, akkor megnézzük a kódokat
  - Van test mode:
    - Mindig ugyan azt a bemenetet leadja / több pre set bemenetet lead
    - Ez nem ér pontot
  - Nekünk kell egy procedurális bemenet generálás minden feladatra
    - Mi írunk egy etalon megoldást és azzal akkor akármilyen bemenethez ki lehet számolni a kimenetet