package com.example.bookmarkbackend;

import org.mybatis.spring.annotation.MapperScan;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

@SpringBootApplication
@MapperScan("com.example.bookmarkbackend.mapper")
// 删掉多余的 @ComponentScan 注解！
public class BookmarkBackendApplication {
    public static void main(String[] args) {
        SpringApplication.run(BookmarkBackendApplication.class, args);
    }
}
