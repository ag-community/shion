drop table `stats`;

alter table `player`
    add column rating float not null default 1000,
    add column uncertainty float not null default 333.33333;