# rust_tunnel

tcp reverse tunnel test

## tags(develop stage)

- step 1 : default TCP reverse tunnel o
- step 2 : multiplexing reverse tunnel or tunnel add o
- step 3 : Asynchrony TCP reverse tunnel x
- step 4 : Test after connecting the db server to the reverse tunnel client
- step 5 : Test after connecting the web client to the reverse tunnel server

문제 : client에서 메세지를 보내고 server에서 반환값을 보내는데 server에서 딜레이가 걸리면 client에서 보낸 메세지가 쌓여 겹치는 경우가 발생한다.
client에서 메세지를 보내고 다시 돌려받기전까지는 해당 tunnel을 막아야한다.

터널은 단일 client, server에 종속된다
메세지를 입력받을때 client Id, server id값을 같이 받아야 할것
client id값을 같이 보내면 server에서는 client id 값에 연결되어있는 server id를 찾아서 데이터를 가져와야함

연결을 추가하는 로직이 들어오면 추가되야함
ex) client id 30(30은 처음들어오는 id값)

db에 요청 server에서 db에 요청?
