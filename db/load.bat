docker-compose exec -T db psql -U admin postgres < down.sql
docker-compose exec -T db psql -U admin clerk_db < up.sql
