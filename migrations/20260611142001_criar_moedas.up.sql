create table if not exists moedas (
    id serial not null,
    nome text not null unique,
    simbolo varchar(10) not null,
    valor decimal(20, 10) not null,
    inclusao_data timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
	alteracao_data timestamp NULL,
	inclusao_usuario text NOT NULL,
	alteracao_usuario text NULL,
	CONSTRAINT pk_moedas PRIMARY KEY (id)
);