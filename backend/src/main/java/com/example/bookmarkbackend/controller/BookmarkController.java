package com.example.bookmarkbackend.controller;

import com.example.bookmarkbackend.entity.Bookmark;
import com.example.bookmarkbackend.service.BookmarkService;
import com.example.bookmarkbackend.util.Result;
import org.springframework.web.bind.annotation.*;

import javax.annotation.Resource;
import java.util.List;

@RestController
@RequestMapping("/bookmark")
public class BookmarkController {

    @Resource
    private BookmarkService bookmarkService;

    @GetMapping("/list")
    public Result<List<Bookmark>> list(Long userId) {
        List<Bookmark> list = bookmarkService.lambdaQuery()
                .eq(Bookmark::getUserId, userId)
                .list();
        return Result.success(list);
    }

    @PostMapping("/add")
    public Result<Bookmark> add(@RequestBody Bookmark bookmark) {
        bookmarkService.saveOrUpdate(bookmark);
        return Result.success(bookmark);
    }

    @PostMapping("/delete")
    public Result delete(@RequestBody Bookmark bookmark) {
        bookmarkService.removeById(bookmark.getId());
        return Result.success();
    }
}