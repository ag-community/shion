create table `stats` (
    player_id int unsigned not null primary key,
    rating float not null default 1000,
    uncertainty float not null default 333.33333,
    wins int unsigned not null default 0,
    losses int unsigned not null default 0,
    total_frags int signed not null default 0,
    total_deaths int signed not null default 0,
    constraint fk_stats_player
        foreign key (player_id) references `player`(id)
        on delete no action on update no action
);

alter table `player`
    drop column rating,
    drop column uncertainty;