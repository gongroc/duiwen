create table if not exists `Folder`
(
    `id`         integer primary key autoincrement not null,
    `title`      text                              not null,
    `intro`      text,
    `created_at` timestamp                         not null default current_timestamp,
    `updated_at` timestamp                         not null default current_timestamp
);

create table if not exists `FolderRelation`
(
    `id`         integer primary key autoincrement not null,
    `start_id`   integer,
    `end_id`     integer                           not null,
    `title`      text,
    `created_at` timestamp                         not null default current_timestamp
);

create table if not exists `Drawer`
(
    `id`         integer primary key autoincrement not null,
    `title`      text                              not null,
    `intro`      text,
    `idx`        int                               not null default 0,
    `current`    text                              not null default 'false',
    `created_at` timestamp                         not null default current_timestamp,
    `updated_at` timestamp                         not null default current_timestamp
);

create table if not exists `FolderDrawer`
(
    `id`         integer primary key autoincrement not null,
    `folder_id`  integer                           not null,
    `drawer_id`  integer                           not null,
    `created_at` timestamp                         not null default current_timestamp
);

create table if not exists `Article`
(
    `id`         integer primary key autoincrement not null,
    `title`      text,
    `content`    text                              not null default '',
    `intro`      text                              not null default '',
    `editing`    text                              not null default 'true',
    `created_at` timestamp                         not null default current_timestamp,
    `updated_at` timestamp                         not null default current_timestamp
);

create table if not exists `ArticleFavorite`
(
    `id`         integer primary key autoincrement not null,
    `article_id` integer                           not null,
    `timestamp`  timestamp                         not null default current_timestamp
);

create table if not exists `ArticleFolder`
(
    `id`         integer primary key autoincrement not null,
    `folder_id`  integer                           not null,
    `article_id` integer                           not null,
    `remark`     text                              not null default '',
    `created_at` timestamp                         not null default current_timestamp
);


create table if not exists `Sequence`
(
    `id`            integer primary key autoincrement not null,
    `folder_id`     integer                           not null,
    `relation_id`   integer                           not null,
    `relation_type` text                              not null,
    `target_id`     integer                           not null,
    `title`         text                              not null,
    `idx`           integer                           not null default 0,
    `timestamp`     timestamp                         not null default current_timestamp
);

create table if not exists `Record`
(
    `id`          integer primary key autoincrement not null,
    `target_id`   integer                           not null,
    `target_type` text                              not null,
    `created_at`  timestamp                         not null default current_timestamp
);
