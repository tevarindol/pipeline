#let fonts = ("Times New Roman", "Liberation Serif", "DejaVu Serif")
#let mono = ("Courier New", "Liberation Mono", "DejaVu Sans Mono")

#set text(font: fonts, size: 14pt, lang: "uk", hyphenate: false)
#set page(paper: "a4", margin: (left: 3cm, right: 1.5cm, top: 2cm, bottom: 2cm))
#set par(justify: true, leading: 1.5em, first-line-indent: 1.25cm, spacing: 1.5em)
#set figure.caption(separator: [ -- ])
#show figure.caption: set text(size: 12pt)
#show heading.where(level: 1): it => align(center, text(weight: "bold", it))
#show raw.where(block: true): set text(font: mono, size: 7.5pt)
#show raw.where(block: true): set par(justify: false, first-line-indent: 0pt, leading: 0.65em)
#show figure.where(kind: "listing"): set block(breakable: true)

#let listing(f, lang: "rust") = {
  let name = f.split("/").slice(1).join("/")
  figure(raw(lang: lang, block: true, read(f)), kind: "listing", supplement: [Лістинг], caption: name)
}

// ---------- Титульна сторінка ----------
#set par(first-line-indent: 0pt, leading: 0.9em, spacing: 0.9em, justify: false)
#align(center)[
  МІНІСТЕРСТВО ОСВІТИ І НАУКИ УКРАЇНИ \
  Національний технічний університет України \
  «Київський політехнічний інститут імені Ігоря Сікорського» \
  Факультет інформатики та обчислювальної техніки \
  Кафедра обчислювальної техніки
]
#v(1fr)
#align(center)[
  #text(size: 16pt, weight: "bold")[Лабораторна робота №6] \
  #v(1.2em)
  з дисципліни \
  «Об’єктно-орієнтоване програмування» \
  #v(1.2em)
  на тему \
  «Побудування програмної системи з множини об’єктів, керованих повідомленнями»
]
#v(1fr)
#grid(
  columns: (1fr, 1fr),
  align: (left, right),
  [
    Виконав: \
    студент групи ІМ-55 \
    Стеблевець Олександр Олександрович \
    Номер в списку – 26
  ],
  [Перевірив: \ Рекечинський Д.О.],
)
#v(2.5em)
#align(center)[Київ 2026]

#pagebreak()

// ---------- Основна частина ----------
#set page(numbering: "1")
#set par(first-line-indent: 1.25cm, leading: 1.5em, spacing: 1.5em, justify: true)

= Мета роботи

Отримати вміння та навички використовувати засоби обміну інформацією та запрограмувати взаємодію незалежно працюючих програмних компонентів.

= Завдання та варіант

Номер у журналі – 26, тому номер варіанта завдання: 26 mod 4 = 2. Необхідно створити три незалежні програми: Lab6, Object2 та Object3. Програма Lab6 приймає параметри n, Min і Max та керує іншими програмами. Object2 генерує вектор із n дробових чисел у заданому діапазоні, відображає його та записує в текстовому форматі в буфер обміну. Object3 зчитує вектор із буфера, сортує його та відображає результат.

Програма Lab6 знаходить уже запущені об’єкти або запускає їх, забезпечує послідовний обмін повідомленнями та дозволяє повторно виконувати завдання. При завершенні Lab6 надсилає об’єктам команди завершення.

= Лістинги програми

Обмін пакетами між програмами реалізовано через іменовані локальні сокети за допомогою бібліотеки interprocess. Спільна бібліотека common містить типи пакетів, засоби обміну повідомленнями та роботи з буфером обміну.

== Lab6

#listing("../lab6/build.rs")
#listing("../lab6/src/main.rs")
#listing("../lab6/src/manager.rs")
#listing("../lab6/src/ui.rs")
#listing("../lab6/src/app.slint", lang: "text")

== Object2

#listing("../object2/build.rs")
#listing("../object2/src/main.rs")
#listing("../object2/src/worker.rs")
#listing("../object2/src/ui.rs")
#listing("../object2/src/app.slint", lang: "text")

== Object3

#listing("../object3/build.rs")
#listing("../object3/src/main.rs")
#listing("../object3/src/worker.rs")
#listing("../object3/src/ui.rs")
#listing("../object3/src/app.slint", lang: "text")

== Спільна бібліотека

#listing("../common/src/lib.rs")
#listing("../common/src/communicate.rs")
#listing("../common/src/packets.rs")
#listing("../common/src/clipboard.rs")

= Схема послідовності надсилання та обробки повідомлень

Після введення параметрів Lab6 послідовно надсилає команди об’єктам і очікує підтвердження їх виконання. На схемі наведено успішне виконання завдання.

#figure(
  image("media/lab6-diagram.png", width: 100%),
  supplement: [Рисунок],
  caption: [Схема послідовності надсилання та обробки повідомлень],
)

= Скріншоти

#figure(
  image("media/lab6-main.png", width: 80%),
  supplement: [Рисунок],
  caption: [Головне вікно Lab6 після виконання завдання],
)

#figure(
  image("media/lab6-configure.png", width: 80%),
  supplement: [Рисунок],
  caption: [Вікно введення параметрів вектора],
)

#figure(
  image("media/lab6-object2.png", width: 80%),
  supplement: [Рисунок],
  caption: [Вікно Object2 зі згенерованим вектором],
)

#figure(
  image("media/lab6-object3.png", width: 80%),
  supplement: [Рисунок],
  caption: [Вікно Object3 з відсортованим вектором],
)

= Висновки

Реалізував систему з трьох незалежних програм, керованих повідомленнями. Організував обмін командами через іменовані локальні сокети та передавання вектора через буфер обміну. Отримав практичний досвід взаємодії програмних компонентів, автоматичного запуску програм і послідовної обробки повідомлень.
