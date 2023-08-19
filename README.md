## start docker 
```docker-compose up -d```

## Diesel setup
```docker-compose exec app diesel setup```

## Diesel create migrations using docker
```docker-compose exec app diesel migration generate create_rustaceans```

## Diesel run migrations using docker
```docker-compose exec app diesel migration run```

## Start rocket server using docker
```docker-compose exec app cargo run```