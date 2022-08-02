create table genre (
	id serial primary key,
	name text
);

create table media (
	id serial primary key,
	title text not null,
	album text,
	location text not null unique,
	genreid int references genre(id)
);

create table artists (
	id serial primary key,
	name text
);

create table media_artist (
	mediaid int references media(id),
	artistid int references artists(id),
	primary key (mediaid, artistid)
);
