export $(xargs <.env)
export DATABASE_URL=postgres://$SCHLEPWISE_POSTGRES_USER:$SCHLEPWISE_POSTGRES_HOST@$SCHLEPWISE_POSTGRES_HOST:$SCHLEPWISE_POSTGRES_PORT/$SCHLEPWISE_POSTGRES_DB_NAME
diesel migration run
