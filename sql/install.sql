create table genre (
	id serial primary key,
	name text
);

create table media (
	id serial primary key,
	title text not null,
	artist text,
	album text,
	location text not null unique,
	genreid int references genre(id)
);
