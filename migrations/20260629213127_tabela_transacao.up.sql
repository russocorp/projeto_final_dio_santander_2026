create table if not exists transacoes (
    id serial not null,
    id_usuarios int not null,
    id_moedas int not null,
    valor_compra decimal(20, 10) not null,
    data_transacao date not null,
    quantidade decimal(20, 10) not null,
    inclusao_data timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
	alteracao_data timestamp NULL,
	inclusao_usuario text NOT NULL,
	alteracao_usuario text NULL,
	CONSTRAINT pk_transacoes PRIMARY KEY (id)
);