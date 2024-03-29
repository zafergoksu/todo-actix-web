drop table if exists todo_list;
drop table if exists todo_item;

create table todo_list (
    id serial primary key,
    title varchar(15)
);

create table todo_item (
    id serial primary key,
    title varchar(15),
    checked boolean not null default false,
    list_id integer not null,
    foreign key (list_id) references todo_list(id)
);

insert into todo_list (title) values ('List 1'), ('List 2');

insert into todo_item (title, list_id) values ('Item 1', 1), ('Item 2', 2), ('Item 3', 2);
