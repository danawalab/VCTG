# VCTG (Virtual Cryptocurrency Trading Game)

해당 프로젝트는 다나와 RUST 스터디 모임에서 하는 토이 프로젝트입니다.
VCTG는 가상 주식 거래 게임을 벤치마킹해서 가상 암호화폐 거래 게임으로
실제 블록체인 기술 구현이 아닌 비슷하게 구현만 하는거고 주 목적은 RUST 문법 익히는데 있습니다.

## requirement
rustup == 1.24.3
rustc  == 1.62.0
mysql  == 22.2.0 (https://docs.rs/mysql/latest/mysql/)

## 개발 시 필요 내용
- RDBMS(maria) - 도커로 실행(docker pull mariadb:10.9.2)
docker run -d --name mariadb -p 3306:3306 --env MARIADB_USER=user --env MARIADB_PASSWORD=password --env MARIADB_ROOT_PASSWORD=root mariadb:10.9.2
- 서버 개발
Port: 7878
branch -> server1 / server2
- 클라이언트 개발
branch -> client1 / client2

### 개발 가이드 (진행하면서 변경 될 수 있음)
- src/main.rs 
- src/database/mysql.rs (database connection 라이브러리 파일)
- 나머지는 폴더를 만들어 아래 위치 시킨다
- 서버 구현은 tcp를 통하여 구현한다.(https://rinthel.github.io/rust-lang-book-ko/ch20-01-single-threaded.html)

## 운영
- RDBMS(maria) - 도커로 실행(docker pull mariadb:10.9.2)
docker run -d --name mariadb -p 3306:3306 --env MARIADB_USER=user --env MARIADB_PASSWORD=password --env MARIADB_ROOT_PASSWORD=root mariadb:10.9.2
- 서버
Port: 7878
branch - server
- 클라이언트
branch - client
