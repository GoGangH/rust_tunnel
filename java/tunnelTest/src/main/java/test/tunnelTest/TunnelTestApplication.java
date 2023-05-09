package test.tunnelTest;

import jakarta.annotation.PreDestroy;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;


@SpringBootApplication
public class TunnelTestApplication {
	private static SshTunneling tunnel;

	public TunnelTestApplication() {
		tunnel = new SshTunneling().init( res->{
			if(!res) {
				System.out.println("포트포워딩 실패, 프로그램을 종료 합니다.");
				System.exit(0);
			}
		});
	}

	public static void main(String[] args) {
		SpringApplication.run(TunnelTestApplication.class, args);
	}

	@PreDestroy
	public void end() {
		try {
			tunnel.shutdown();
		} catch (Exception e) {
			e.printStackTrace();
		}
	}
}