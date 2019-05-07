Robo Instructus Translations
============================
Translations for programming puzzle game Robo Instructus.
![](https://user-images.githubusercontent.com/2331607/57169312-dd644400-6dfd-11e9-948c-f163f1e3625e.png)

## About
This project exists to provide a way for open source community translations built on machine translations to be used in game. As a solo dev with limited resources I may not be able to fund many professional translations, so this can provide a route to a translated game that otherwise wouldn't exist.

## Want to help?
If you are multilingual and want to improve the game for your language(s) then please do.

### Interface, menus, tutorial overlays
Shorter translations like menu buttons are arranged in pairs of English then translated lines of text:
```
Profile
Profil

Delete profile
Usuń profil
```

Take a look at the existing `en-replace.LANGUAGE.pairs` files:

#### Machine translations needing review
These translations have been created using google translate and need to be improved before being used in game.
* български език [en-replace.bg.pairs](./en-replace.bg.pairs)
* 简体中文 [en-replace.zh-CN.pairs](./en-replace.zh-CN.pairs)
* čeština [en-replace.cs.pairs](./en-replace.cs.pairs)
* Dansk [en-replace.da.pairs](./en-replace.da.pairs)
* Suomi [en-replace.fi.pairs](./en-replace.fi.pairs)
* Ελληνικά [en-replace.el.pairs](./en-replace.el.pairs)
* Magyar [en-replace.hu.pairs](./en-replace.hu.pairs)
* Italiano [en-replace.it.pairs](./en-replace.it.pairs)
* 日本語 [en-replace.ja.pairs](./en-replace.ja.pairs)
* 한국어 [en-replace.ko.pairs](./en-replace.ko.pairs)
* Norsk [en-replace.no.pairs](./en-replace.no.pairs)
* Português-Brasil [en-replace.pt-BR.pairs](./en-replace.pt-BR.pairs)
* Română [en-replace.ro.pairs](./en-replace.ro.pairs)
* Español-España [en-replace.es.pairs](./en-replace.es.pairs)
* Svenska [en-replace.sv.pairs](./en-replace.sv.pairs)
* ไทย [en-replace.th.pairs](./en-replace.th.pairs)
* Türkçe [en-replace.tr.pairs](./en-replace.tr.pairs)
* Українська [en-replace.uk.pairs](./en-replace.uk.pairs)
* Tiếng Việt [en-replace.vi.pairs](./en-replace.vi.pairs)

#### Community improved translations
These languages will be made available in game, though are only partial and may still need tweaks.
* Русский [en-replace.ru.pairs](./en-replace.ru.pairs)
* Nederlands [en-replace.nl.pairs](./en-replace.nl.pairs)
* Português [en-replace.pt.pairs](./en-replace.pt.pairs)
* Polski [en-replace.pl.pairs](./en-replace.pl.pairs)
* 繁體中文 [en-replace.zh-TW.pairs](./en-replace.zh-TW.pairs)
* Français [en-replace.fr.pairs](./en-replace.fr.pairs)
* Deutsch [en-replace.de.pairs](./en-replace.de.pairs)

If you see a line that could be improved edit it and raise a pull request.

### New languages
If you'd like to see support for a new language please [raise an issue](https://github.com/big-ab-games/robo-instructus-translation/issues/new).

## Test
`cargo test --all --all-features`
