begin;
create table users (
	id serial primary key,
	username varchar(100),
	password varchar(200)
);

insert into users values 
	(1, 'admin', 'password');

create table authtokens (
    value varchar(36) primary key,
    created timestamp not null,
    user_id integer not null unique,
    foreign key (user_id) references users (id)
);

create table projects (
	id serial primary key,
	title varchar(100),
	owner_id integer not null,
	foreign key (owner_id) references users (id)
);

insert into projects (id, title, owner_id) values
	(1, 'Project 1', 1),
	(2, 'Project 2', 1);

create table tasks (
	id serial primary key, 
	title varchar(100), 
	project_id integer not null,
	foreign key (project_id) references projects (id)
);

insert into tasks values
	(1, 'Task 1', 1),
	(2, 'Task 2', 1);
commit;
