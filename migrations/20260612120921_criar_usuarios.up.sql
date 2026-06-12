create table if not exists usuarios (
    id serial not null,    
    nome text not null,
    user_name text not null,
    hashed_password text not null,
    is_admin boolean not null default false,
    inclusao_data timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
	alteracao_data timestamp NULL,
	inclusao_usuario text NOT NULL,
	alteracao_usuario text NULL,
	CONSTRAINT pk_usuarios PRIMARY KEY (id),
    CONSTRAINT uq_usuarios_username UNIQUE (user_name)
);