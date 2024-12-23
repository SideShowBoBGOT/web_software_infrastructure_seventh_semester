CREATE TABLE students (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    surname VARCHAR(255) NOT NULL,
    group_id INT NOT NULL,
    image_data BYTEA,
    image_type VARCHAR(30)
);

INSERT INTO students (name, surname, group_id) VALUES
    ('Сергій', 'Панченко', 0),
    -- ('Андрій', 'Ющенко', 0),
    -- ('Іван', 'Трикош', 0),
    -- ('Іван', 'Ляля', 0),
    -- ('Кирил', 'Сідак', 0),
    -- ('Юрій', 'Рябов', 0),
    -- ('Дмитро', 'Книш', 0),
    -- ('Дмитро', 'Кузьменков', 0),
    -- ('Олександр', 'Головня', 0),
    -- ('Олександр', 'Печковський', 0),
    -- ('Владислав', 'Прищепа', 0),
    -- ('Іман Айман', 'Хамад', 0),
    -- ('Олексій', 'Савенко', 0),
    -- ('Владислав', 'Лесів', 0),
    -- ('Даниїл', 'Гіжицький', 0),
    -- ('Федір', 'Тихонов', 0),
    -- ('Кирило', 'Гуськов', 0),
    -- ('Іван', 'Боровков', 0),
    -- ('Андрій', 'Калашніков', 0),
    -- ('Ілона', 'Дякунчак', 0),
    -- ('Олександра', 'Друзенко', 0),
    -- ('Дмитро', 'Кравченко', 0),
    -- ('Віктор', 'Лошак', 0),
    -- ('Вікторія', 'Фукс', 0),
    -- ('Ігор', 'Веремчук', 0),
    -- ('Владислав', 'Головатюк', 0),
    -- ('Андрій', 'Лисенко', 0),
    -- ('Михайло', 'Дідур', 0),
    
    ('Тетяна', 'Луговець', 1),
    ('Михайло', 'Мельник', 1),
    -- ('Олексій', 'Горобець', 1),
    -- ('Вадим', 'Волков', 1),
    -- ('Ярослав', 'Орищенко', 1),
    -- ('Владислав', 'Логвиненко', 1),
    -- ('Єгор', 'Васильєв', 1),
    -- ('Данило', 'Титаренко', 1),
    -- ('Віталій', 'Піонтківський', 1),
    -- ('Родіон', 'Скорик', 1),
    -- ('Микола', 'Спаських', 1),
    -- ('Даніїл', 'Йолкін', 1),
    -- ('Ганна', 'Кушнір', 1),
    -- ('Данило', 'Шоман', 1),
    -- ('Дмитро', 'Стецун', 1),
    -- ('Анастасія', 'Бондарчук', 1),
    -- ('Денис', 'Дулов', 1),
    -- ('Максим', 'Бобрик', 1),
    -- ('Андрій', 'Сімчук', 1),
    -- ('Григорій', 'Авчаров', 1),
    -- ('Олег', 'Басараб', 1),
    -- ('Артем', 'Єльчанінов', 1),
    -- ('Володимир', 'Казаков', 1),
    -- ('Катерина', 'Доброхотова', 1),
    -- ('Євгеній', 'Чекаленко', 1),
    -- ('Роман', 'Гаптар', 1),

    -- ('Ганна', 'Шиманська', 2),
    -- ('Андрій', 'Качмар', 2),
    -- ('Микита', 'Кисельов', 2),
    -- ('Лідія', 'Макарчук', 2),
    -- ('Юрій', 'Сергієнко', 2),
    -- ('Діана', 'Грицина', 2),
    ('Денис', 'Бабіч', 2),
    -- ('Олександр', 'Дем`янчук', 2),
    -- ('Дмитро', 'Жмайло', 2),
    -- ('Віталій', 'Музичук', 2),
    -- ('Варвара', 'Головач', 2),
    -- ('Анастасія', 'Лисенко', 2),
    -- ('Максим', 'Бондаренко', 2),
    -- ('Вартан', 'Карамян', 2),
    -- ('Ілля', 'Пархомчук', 2),
    -- ('Артур', 'Саіян', 2),
    -- ('Валерія', 'Радзівіло', 2),
    -- ('Ігор', 'Петров', 2),
    -- ('Микита', 'Павленко', 2),
    -- ('Олексій', 'Бабашев', 2),
    -- ('Олександр', 'Паламарчук', 2),
    -- ('Станіслав', 'Вдовиченко', 2),
    -- ('Мурат', 'Ал Хадам', 2),
    -- ('Євген', 'Недельчев', 2),
    -- ('Назар', 'Ковалик', 2),
    -- ('Максим', 'Лопоша', 2),
    -- ('Родіон', 'Григоренко', 2),
    -- ('Владислав', 'Дейнега', 2),
    -- ('Анастасія', 'Шевцова', 2),
    -- ('Микита', 'Криворук', 2),
    -- ('Вадим', 'Крупосій', 2),
    -- ('Дмитро', 'Замковий', 2),
    -- ('Віталій', 'Нещерет', 2),

    -- ('Артем', 'Хільчук', 3),
    -- ('Кирило', 'Волинець', 3),
    -- ('Соня', 'Кондрацька', 3),
    -- ('Владислав', 'Тонконог', 3),
    -- ('Олександр', 'Медвідь', 3),
    -- ('Артем', 'Химич', 3),
    -- ('Владислав', 'Борисик', 3),
    -- ('Валерій', 'Поліщук', 3),
    -- ('Даніїл', 'Богун', 3),
    -- ('Мадіна', 'Аджигельдієва', 3),
    -- ('Антон', 'Щербацький', 3),
    -- ('Владислав', 'Чорній', 3),
    -- ('Олександр', 'Журбелюк', 3),
    -- ('Денис', 'Шляхтун', 3),
    -- ('Олексій', 'Легеза', 3),
    -- ('Дмитро', 'Мочалов', 3),
    -- ('Метін', 'Шабанов', 3),
    -- ('Олексій', 'Прокопенко', 3),
    -- ('Ілля', 'Рибалка', 3),
    -- ('Ярослав', 'Гордієнко', 3),
    -- ('Вадим', 'Костін', 3),
    -- ('Кирило', 'Лазьов', 3),
    -- ('Марія', 'Коваленко', 3),
    -- ('Іван', 'Дацьо', 3),
    -- ('Дмитро', 'Плугатирьов', 3),
    -- ('Андрій', 'Мєшков', 3),
    -- ('Ірина', 'Куманецька', 3),
    -- ('Олексій', 'Лазюта', 3),
    -- ('Олександр', 'Гуменюк', 3),
    -- ('Дмитро', 'Буяло', 3),
    -- ('Данило', 'Мельник', 3),
    ('Владислав', 'Ткач', 3);