create table if not exists users (
	id serial primary key,
	username varchar(100),
	password varchar(200)
);

insert into users (id, username, password) values (1, 'admin', 'password');
