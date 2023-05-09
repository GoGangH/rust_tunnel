package test.dbtest.member;

import test.dbtest.member.entity.MemberEntity;
import test.dbtest.member.MemberRepository;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController // JSON 형태 결과값을 반환해줌 (@ResponseBody가 필요없음)
@RequiredArgsConstructor // final 객체를 Constructor Injection 해줌. (Autowired 역할)
@RequestMapping("/v1") // version1의 API
public class MemberController {

    private final MemberRepository memberRepository;

    /**
     * 멤버 조회
     * @return
     */
    @GetMapping("member")
    public List<MemberEntity> findAllMember() {
        return memberRepository.findAll();
    }

    /**
     * 회원가입
     * @return
     */
    @PostMapping("member")
    public MemberEntity signUp() {
        final MemberEntity member = MemberEntity.builder()
                .username("zlzn1020@gmail.com")
                .name("rhrkdgus")
                .build();
        return memberRepository.save(member);
    }
}