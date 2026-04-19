package com.example.bookmarkbackend.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.example.bookmarkbackend.entity.Bookmark;
import com.example.bookmarkbackend.mapper.BookmarkMapper;
import com.example.bookmarkbackend.service.BookmarkService;
import org.springframework.stereotype.Service;

@Service
public class BookmarkServiceImpl extends ServiceImpl<BookmarkMapper, Bookmark> implements BookmarkService {
}