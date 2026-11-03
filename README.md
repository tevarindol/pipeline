# Лабораторна робота №6. Побудування програмної системи з множини об’єктів, керованих повідомленнями

Номер у журналі 26, варіант завдання **2**.

Система складається з трьох незалежних програм:

- **Lab6** приймає параметри `n`, `Min`, `Max` і керує іншими програмами: знаходить запущені або запускає їх та надсилає команди.
- **Object2** генерує вектор із `n` дробових чисел у діапазоні від `Min` до `Max`, показує його у вікні та записує в текстовому форматі в буфер обміну.
- **Object3** зчитує вектор із буфера обміну, сортує його та показує результат у своєму вікні.

Обмін командами та відповідями відбувається через іменовані локальні сокети `interprocess`, а вектор передається через системний буфер обміну. Після виконання завдання його можна повторити; при закритті Lab6 обидва об’єкти отримують команду завершення.

## Запуск

Потрібне графічне середовище з доступним буфером обміну. Спочатку необхідно зібрати всі три програми, щоб виконувані файли об’єктів були поруч із Lab6:

```sh
cargo build --workspace
cargo run -p lab6
```

## Діаграма програми

```mermaid
flowchart LR
    subgraph lab6["lab6"]
        lmain["fn main()"]
        lui["fn ui::configure()"]
        lrun["fn manager::run()"]
        lapp["App<br/>status, busy<br/>open_parameters()"]
        params["Parameters<br/>execute(n, min, max)"]
        command["enum ManagerCommand<br/>Execute, Shutdown"]
        manager["struct Manager<br/>communicate, next_id<br/>next_id(), wait_for()<br/>ensure_worker(), execute()"]
        lmain --> lui
        lmain --> lrun
        lui -. "налаштовує" .-> lapp
        lui -. "налаштовує" .-> params
        lui -- "надсилає" --> command
        command -- "mpsc" --> lrun
        lrun --> manager
        lrun -. "оновлює стан" .-> lapp
    end

    subgraph object2["object2"]
        o2main["fn main()"]
        o2configure["fn ui::configure()"]
        o2run["fn worker::run()"]
        o2show["fn ui::show_values()"]
        o2quit["fn ui::quit()"]
        o2app["App<br/>values"]
        o2main --> o2configure
        o2main --> o2run
        o2run --> o2show
        o2run --> o2quit
        o2configure -. "налаштовує" .-> o2app
        o2show -. "оновлює" .-> o2app
    end

    subgraph object3["object3"]
        o3main["fn main()"]
        o3configure["fn ui::configure()"]
        o3run["fn worker::run()"]
        o3show["fn ui::show_values()"]
        o3quit["fn ui::quit()"]
        o3app["App<br/>values"]
        o3main --> o3configure
        o3main --> o3run
        o3run --> o3show
        o3run --> o3quit
        o3configure -. "налаштовує" .-> o3app
        o3show -. "оновлює" .-> o3app
    end

    subgraph common["common"]
        create["fn communicate::create()"]
        validate["fn validate()"]
        format["fn format_values()"]
        communicate["struct Communicate<br/>send_packet()<br/>receive_packet()<br/>has_new_packets()"]
        packet["enum Packet<br/>Ping, Generate, Sort<br/>Shutdown, Finished"]
        clipboard["struct Clipboard<br/>write_vector()<br/>read_vector()"]
        create --> communicate
        communicate -. "передає" .-> packet
    end

    lui --> validate
    lrun --> create
    manager -- "містить" --> communicate
    manager -. "формує та отримує" .-> packet
    o2main --> create
    o3main --> create
    o2run --> communicate
    o3run --> communicate
    o2run -. "обробляє" .-> packet
    o3run -. "обробляє" .-> packet
    o2run -- "запис" --> clipboard
    o3run -- "читання" --> clipboard
    o2show --> format
    o3show --> format
```
