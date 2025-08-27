create table `player` (
    id int unsigned not null primary key auto_increment,
    steam_id varchar(24) not null unique,
    steam_name varchar(64) not null,
    steam_avatar_url varchar(256) not null,
    rating float not null default 1000,
    uncertainty float not null default 333.33333
);

create table `match` (
    id int unsigned not null primary key auto_increment,
    server_ip varchar(21) not null,
    match_date timestamp not null default current_timestamp(),
    map_name varchar(64) not null
);

create table `match_detail` (
    id int unsigned not null primary key auto_increment,
    player_id int unsigned not null,
    match_id int unsigned not null,
    frags int signed not null,
    deaths int signed not null,
    average_ping int unsigned not null,
    damage_dealt int unsigned not null,
    damage_taken int unsigned not null,
    model varchar(64) not null,
    rating_after_match float not null,
    rating_delta float not null,
    constraint fk_match_detail_player
        foreign key (player_id) references `player`(id)
        on delete no action on update no action,
    constraint fk_match_detail_match
        foreign key (match_id) references `match`(id)
        on delete no action on update no action
);