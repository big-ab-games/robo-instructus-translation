Robo Instructus Translations
============================
Translations for programming puzzle game Robo Instructus.

![](https://user-images.githubusercontent.com/2331607/61449475-ee2c4c80-a94c-11e9-9390-3832f9c7f1e0.png)

## About
This project exists to provide a way for open source community translations to be used in game. As a solo developer with limited resources I'm not able to fund professional translations, so this repository can provide a route to a translated game that otherwise wouldn't exist.

Language | [1. Interface & tutorials](#1-interface-menus-tutorial-overlays "Interface, menus, tutorial overlays") | [2. Store Description](#2-game-store-description "Game description that appears in Steam & other stores.") | [3. Company Messages](#3-company-messages "Messages from Judith.") | [4. Code Primer](#4-code-primer "Bartram's Code Primer.") | [5. Function Docs](#5-function-docs "Function Documentation") | [6. Colony Messages](#6-colony-messages "Colony Messages")
:---: | :---: | :---: | :---: | :---: | :---: | :---:
English _(en)_ | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark:
Русский _(ru)_ | :heavy_check_mark: | :heavy_check_mark: | :black_circle: | :heavy_check_mark: | :heavy_check_mark: | :black_circle:
Deutsch _(de)_ | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :black_circle:
Français _(fr)_ | :large_blue_circle: | :heavy_check_mark: | :black_circle: | :black_circle: | :black_circle: | :black_circle:
简体中文 _(zh-CN)_ | :heavy_check_mark: | :heavy_check_mark: | :black_circle: | :black_circle: | :black_circle: | :black_circle:
繁體中文 _(zh-TW)_ | :heavy_check_mark: | :heavy_check_mark: | :black_circle: | :black_circle: | :black_circle: | :black_circle:
Nederlands _(nl)_ | :large_blue_circle: | :black_circle: | :black_circle: | :black_circle: | :black_circle: | :black_circle:
Svenska _(sv)_ | :large_blue_circle: | :black_circle: | :black_circle: | :black_circle: | :black_circle: | :black_circle:
Polski _(pl)_ | :large_blue_circle: | :black_circle: | :black_circle: | :black_circle: | :black_circle: | :black_circle:
Português-Brasil _(pt-BR)_ | :large_blue_circle: | :heavy_check_mark: | :heavy_check_mark: | :black_circle: | :black_circle: | :black_circle:
Português _(pt)_ | :large_blue_circle: | :black_circle: | :black_circle: | :black_circle: | :black_circle: | :black_circle:
Italiano _(it)_ | :large_blue_circle: | :black_circle: | :black_circle: | :black_circle: | :black_circle: | :black_circle:
Ελληνικά _(el)_ | :large_blue_circle: | :black_circle: | :black_circle: | :black_circle: | :black_circle: | :black_circle:
_others_ | :black_circle: | :black_circle: | :black_circle: | :black_circle: | :black_circle: | :black_circle:

:heavy_check_mark:: _Translated_<br/>
:large_blue_circle:: _Partially translated, some more work to be done / new text to translate_<br/>
:black_circle:: _Not translated, please raise a pull request_<br/>

## Want to help?
If you are multilingual and want to improve the game for your language(s) then please do.

## 1. Interface, menus, tutorial overlays
Shorter translations like menu buttons are arranged in pairs of English then translated lines of text:
```
Profile
Profil

Delete profile
Usuń profil
```

Take a look at the existing `en-replace.LANGUAGE.pairs` files in the `translated-pairs` directory. For example [translated-pairs/en-replace.de.pairs](./translated-pairs/en-replace.de.pairs).

If you see a line that could be improved edit it and raise a pull request.

## 2. Game store description
The game description that appears in Steam & other stores. The original English description is in [store/store-description.en.bbcode](./store/store-description.en.bbcode).

![](https://user-images.githubusercontent.com/2331607/59967068-293d8a80-951d-11e9-92c4-549bbeafe3a8.png)

Improve or create the file `store/store-description.LANGUAGE.bbcode` with translated text. For example [store/store-description.ru.bbcode](./store/store-description.ru.bbcode).

## 3. Company messages
Bit of the game's story delivered as messages from the players boss Judith. These are some of the first bits of text read by the player setting the scene. There are mostly paragraphs of text coming after `#!company` line markers. Try to use the same whitespacing, number of new lines etc.

Update the `company.LANGUAGE.robomarkup` file in the `company` directory. See the English [company/company.en.robomarkup](./company/company.en.robomarkup) file, duplicate this to start a new language translation.

## 4. Code Primer
The fundamentals of how the game's programming language works are explained by Dr. Bartram's "Primer". Each section of the primer is marked with `#!unlock type{primer}` with the text coming after the contents of that section.

Update the `primer.LANGUAGE.robomarkup` file in the `primer` directory. Info: [primer/README](./primer/README.md). See the English [primer/primer.en.robomarkup](./primer/primer.en.robomarkup) file, duplicate this to start a new language translation.

## 5. Function Docs
Each function in the game comes with documentation. These are laid out similarly to the Primer.

Update the `fun.LANGUAGE.robomarkup` file in the `function` directory. Info: [function/README](./function/README.md). See the English [function/fun.en.robomarkup](./function/fun.en.robomarkup) file, duplicate this to start a new language translation.

## 6. Colony messages
Messages between the colonists found gradually throughout the game. There are mostly paragraphs of text coming after `#!unlock` line markers. Try to use the same whitespacing, number of new lines etc.

Update the `colony.LANGUAGE.robomarkup` file in the `colony` directory. See the English [colony/colony.en.robomarkup](./colony/colony.en.robomarkup) file, duplicate this to start a new language translation.

## New languages & translations
If you'd like to see support for a new language please or have questions [raise an issue](https://github.com/big-ab-games/robo-instructus-translation/issues/new).

## Credits
If you help improve the game's translations you'll appear in the game's credits (unless you don't want to). Everyone in the [credits.txt](./credits.txt) is included. _In the game's title screen open the menu and press **Credits** to view them_.
