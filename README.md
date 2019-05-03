Robo Instructus Translations
============================
Translations for programming puzzle game Robo Instructus.
![](https://user-images.githubusercontent.com/2331607/57152106-6feaf000-6dca-11e9-8591-87872cb7e3fa.jpg)

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

* Русский [en-replace.ru.pairs](./en-replace.ru.pairs)
* Polski [en-replace.pl.pairs](./en-replace.pl.pairs)
* Deutsch [en-replace.de.pairs](./en-replace.de.pairs)

If you see a line that could be improved edit it and raise a pull request.

### New languages
If you'd like to see support for a new language please [raise an issue](https://github.com/big-ab-games/robo-instructus-translation/issues/new).

## Test
`cargo test --all --all-features`
