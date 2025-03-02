(module $rle_linestatus_paged.wasm
  (type (;0;) (func (param i32 i32 i32)))
  (type (;1;) (func (param i32 i32 i32) (result i32)))
  (type (;2;) (func (param i32) (result i32)))
  (type (;3;) (func (param i32 i32)))
  (type (;4;) (func (param i32 i32 i32 i32 i32 i32 i32) (result i32)))
  (type (;5;) (func (param i32)))
  (type (;6;) (func))
  (func $_ZN5alloc7raw_vec11finish_grow17h9c2a4df959a4cf6dE (type 0) (param i32 i32 i32)
    (local i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 2
          i32.load offset=4
          i32.eqz
          br_if 0 (;@3;)
          block  ;; label = @4
            local.get 2
            i32.load offset=8
            local.tee 3
            br_if 0 (;@4;)
            i32.const 0
            i32.load8_u offset=1049276
            drop
            br 2 (;@2;)
          end
          local.get 2
          i32.load
          local.get 3
          local.get 1
          call $__rust_realloc
          local.set 2
          br 2 (;@1;)
        end
        i32.const 0
        i32.load8_u offset=1049276
        drop
      end
      local.get 1
      call $_ZN4talc4talc13Talc$LT$O$GT$6malloc17h3aea4463430a8bb9E
      local.set 2
    end
    local.get 0
    local.get 1
    i32.store offset=8
    local.get 0
    local.get 2
    i32.const 1
    local.get 2
    select
    i32.store offset=4
    local.get 0
    local.get 2
    i32.eqz
    i32.store)
  (func $__rust_realloc (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              i32.const -1
                              local.get 2
                              local.get 1
                              i32.ne
                              local.get 2
                              local.get 1
                              i32.lt_u
                              select
                              i32.const 255
                              i32.and
                              br_table 12 (;@1;) 1 (;@12;) 0 (;@13;)
                            end
                            block  ;; label = @13
                              block  ;; label = @14
                                local.get 0
                                local.get 1
                                i32.add
                                i32.const 3
                                i32.add
                                i32.const -4
                                i32.and
                                local.tee 1
                                i32.load
                                local.tee 3
                                local.get 1
                                i32.gt_u
                                br_if 0 (;@14;)
                                local.get 3
                                local.set 4
                                br 1 (;@13;)
                              end
                              local.get 3
                              i32.load
                              local.set 4
                              local.get 3
                              local.set 1
                            end
                            block  ;; label = @13
                              local.get 1
                              local.get 4
                              i32.const -4
                              i32.and
                              local.tee 5
                              i32.const 8
                              i32.add
                              local.tee 6
                              local.get 0
                              local.get 2
                              i32.add
                              i32.const 3
                              i32.add
                              i32.const -4
                              i32.and
                              local.tee 3
                              local.get 6
                              local.get 3
                              i32.gt_u
                              select
                              local.tee 2
                              i32.sub
                              i32.const 12
                              i32.ge_u
                              br_if 0 (;@13;)
                              local.get 1
                              local.set 2
                              br 11 (;@2;)
                            end
                            local.get 2
                            i32.const 4
                            i32.add
                            local.set 7
                            local.get 1
                            i32.const 4
                            i32.add
                            local.set 6
                            local.get 4
                            i32.const 2
                            i32.and
                            i32.eqz
                            br_if 9 (;@3;)
                            local.get 1
                            i32.load offset=12
                            local.tee 4
                            i32.const 64
                            i32.lt_u
                            br_if 1 (;@11;)
                            block  ;; label = @13
                              local.get 4
                              i32.const 128
                              i32.ge_u
                              br_if 0 (;@13;)
                              local.get 4
                              i32.const 3
                              i32.shr_u
                              i32.const 5
                              i32.add
                              local.set 8
                              br 9 (;@4;)
                            end
                            local.get 4
                            i32.const 30
                            local.get 4
                            i32.clz
                            local.tee 8
                            i32.sub
                            i32.shr_u
                            local.get 8
                            i32.const 1
                            i32.shl
                            i32.sub
                            i32.const 67
                            i32.add
                            local.tee 8
                            i32.const 63
                            local.get 8
                            i32.const 63
                            i32.lt_u
                            select
                            local.set 8
                            br 8 (;@4;)
                          end
                          local.get 0
                          local.get 1
                          i32.add
                          i32.const 3
                          i32.add
                          i32.const -4
                          i32.and
                          local.tee 3
                          local.get 0
                          local.get 2
                          i32.add
                          i32.const 3
                          i32.add
                          i32.const -4
                          i32.and
                          local.tee 4
                          i32.eq
                          br_if 10 (;@1;)
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 3
                              i32.load
                              local.tee 7
                              local.get 3
                              i32.gt_u
                              br_if 0 (;@13;)
                              local.get 3
                              local.set 6
                              local.get 7
                              local.set 5
                              br 1 (;@12;)
                            end
                            local.get 7
                            i32.load
                            local.set 5
                            local.get 7
                            local.set 6
                          end
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                local.get 4
                                local.get 6
                                i32.le_u
                                br_if 0 (;@14;)
                                block  ;; label = @15
                                  local.get 5
                                  i32.const 2
                                  i32.and
                                  i32.eqz
                                  br_if 0 (;@15;)
                                  local.get 4
                                  local.get 6
                                  local.get 6
                                  i32.load offset=12
                                  local.tee 8
                                  i32.add
                                  local.tee 7
                                  i32.le_u
                                  br_if 2 (;@13;)
                                end
                                local.get 2
                                call $_ZN4talc4talc13Talc$LT$O$GT$6malloc17h3aea4463430a8bb9E
                                local.tee 2
                                br_if 2 (;@12;)
                                i32.const 0
                                return
                              end
                              local.get 4
                              local.get 6
                              i32.ge_u
                              br_if 12 (;@1;)
                              local.get 4
                              local.get 6
                              i32.store
                              local.get 0
                              return
                            end
                            local.get 8
                            i32.const 64
                            i32.lt_u
                            br_if 2 (;@10;)
                            block  ;; label = @13
                              local.get 8
                              i32.const 128
                              i32.ge_u
                              br_if 0 (;@13;)
                              local.get 8
                              i32.const 3
                              i32.shr_u
                              i32.const 5
                              i32.add
                              local.set 2
                              br 8 (;@5;)
                            end
                            local.get 8
                            i32.const 30
                            local.get 8
                            i32.clz
                            local.tee 2
                            i32.sub
                            i32.shr_u
                            local.get 2
                            i32.const 1
                            i32.shl
                            i32.sub
                            i32.const 67
                            i32.add
                            local.tee 2
                            i32.const 63
                            local.get 2
                            i32.const 63
                            i32.lt_u
                            select
                            local.set 2
                            br 7 (;@5;)
                          end
                          local.get 2
                          local.get 0
                          local.get 1
                          call $memcpy
                          local.set 7
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 3
                              i32.load
                              local.tee 2
                              local.get 3
                              i32.gt_u
                              br_if 0 (;@13;)
                              local.get 2
                              local.set 0
                              br 1 (;@12;)
                            end
                            local.get 2
                            i32.load
                            local.set 0
                            local.get 2
                            local.set 3
                          end
                          local.get 0
                          i32.const -4
                          i32.and
                          local.tee 2
                          i32.const -4
                          i32.add
                          local.tee 4
                          i32.load
                          local.tee 1
                          i32.const 1
                          i32.and
                          br_if 3 (;@8;)
                          local.get 2
                          local.get 1
                          i32.sub
                          local.set 2
                          local.get 1
                          i32.const 64
                          i32.lt_u
                          br_if 2 (;@9;)
                          block  ;; label = @12
                            local.get 1
                            i32.const 128
                            i32.ge_u
                            br_if 0 (;@12;)
                            local.get 1
                            i32.const 3
                            i32.shr_u
                            i32.const 5
                            i32.add
                            local.set 1
                            br 5 (;@7;)
                          end
                          local.get 1
                          i32.const 30
                          local.get 1
                          i32.clz
                          local.tee 4
                          i32.sub
                          i32.shr_u
                          local.get 4
                          i32.const 1
                          i32.shl
                          i32.sub
                          i32.const 67
                          i32.add
                          local.tee 1
                          i32.const 63
                          local.get 1
                          i32.const 63
                          i32.lt_u
                          select
                          local.set 1
                          br 4 (;@7;)
                        end
                        local.get 4
                        i32.const -12
                        i32.add
                        i32.const 2
                        i32.shr_u
                        local.set 8
                        br 6 (;@4;)
                      end
                      local.get 8
                      i32.const -12
                      i32.add
                      i32.const 2
                      i32.shr_u
                      local.set 2
                      br 4 (;@5;)
                    end
                    local.get 1
                    i32.const -12
                    i32.add
                    i32.const 2
                    i32.shr_u
                    local.set 1
                    br 1 (;@7;)
                  end
                  local.get 4
                  local.get 1
                  i32.const 2
                  i32.add
                  i32.store
                  br 1 (;@6;)
                end
                local.get 2
                i32.load offset=4
                local.tee 6
                local.get 2
                i32.load
                local.tee 4
                i32.store
                block  ;; label = @7
                  local.get 4
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 4
                  local.get 6
                  i32.store offset=4
                end
                i32.const 0
                i32.load offset=1049296
                local.get 1
                i32.const 2
                i32.shl
                i32.add
                i32.load
                br_if 0 (;@6;)
                block  ;; label = @7
                  local.get 1
                  i32.const 32
                  i32.lt_u
                  br_if 0 (;@7;)
                  i32.const 0
                  i32.const 0
                  i32.load offset=1049292
                  i32.const 1
                  local.get 1
                  i32.shl
                  i32.xor
                  i32.store offset=1049292
                  br 1 (;@6;)
                end
                i32.const 0
                i32.const 0
                i32.load offset=1049288
                i32.const 1
                local.get 1
                i32.shl
                i32.xor
                i32.store offset=1049288
              end
              local.get 3
              i32.const 4
              i32.add
              local.set 4
              block  ;; label = @6
                local.get 0
                i32.const 2
                i32.and
                i32.eqz
                br_if 0 (;@6;)
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 3
                    i32.load offset=12
                    local.tee 1
                    i32.const 64
                    i32.lt_u
                    br_if 0 (;@8;)
                    block  ;; label = @9
                      local.get 1
                      i32.const 128
                      i32.ge_u
                      br_if 0 (;@9;)
                      local.get 1
                      i32.const 3
                      i32.shr_u
                      i32.const 5
                      i32.add
                      local.set 0
                      br 2 (;@7;)
                    end
                    local.get 1
                    i32.const 30
                    local.get 1
                    i32.clz
                    local.tee 0
                    i32.sub
                    i32.shr_u
                    local.get 0
                    i32.const 1
                    i32.shl
                    i32.sub
                    i32.const 67
                    i32.add
                    local.tee 0
                    i32.const 63
                    local.get 0
                    i32.const 63
                    i32.lt_u
                    select
                    local.set 0
                    br 1 (;@7;)
                  end
                  local.get 1
                  i32.const -12
                  i32.add
                  i32.const 2
                  i32.shr_u
                  local.set 0
                end
                local.get 3
                i32.load offset=8
                local.tee 6
                local.get 3
                i32.load offset=4
                local.tee 3
                i32.store
                block  ;; label = @7
                  local.get 3
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 3
                  local.get 6
                  i32.store offset=4
                end
                block  ;; label = @7
                  i32.const 0
                  i32.load offset=1049296
                  local.get 0
                  i32.const 2
                  i32.shl
                  i32.add
                  i32.load
                  br_if 0 (;@7;)
                  i32.const 8
                  i32.const 12
                  local.get 0
                  i32.const 32
                  i32.lt_u
                  select
                  i32.const 1049280
                  i32.add
                  local.tee 3
                  local.get 3
                  i32.load
                  i32.const 1
                  local.get 0
                  i32.shl
                  i32.xor
                  i32.store
                end
                local.get 4
                local.get 1
                i32.add
                local.set 4
              end
              block  ;; label = @6
                block  ;; label = @7
                  local.get 4
                  local.get 2
                  i32.sub
                  local.tee 1
                  i32.const 64
                  i32.lt_u
                  br_if 0 (;@7;)
                  block  ;; label = @8
                    local.get 1
                    i32.const 128
                    i32.ge_u
                    br_if 0 (;@8;)
                    local.get 1
                    i32.const 3
                    i32.shr_u
                    i32.const 5
                    i32.add
                    local.set 3
                    br 2 (;@6;)
                  end
                  local.get 1
                  i32.const 30
                  local.get 1
                  i32.clz
                  local.tee 0
                  i32.sub
                  i32.shr_u
                  local.get 0
                  i32.const 1
                  i32.shl
                  i32.sub
                  i32.const 67
                  i32.add
                  local.tee 0
                  i32.const 63
                  local.get 0
                  i32.const 63
                  i32.lt_u
                  select
                  local.set 3
                  br 1 (;@6;)
                end
                local.get 1
                i32.const -12
                i32.add
                i32.const 2
                i32.shr_u
                local.set 3
              end
              block  ;; label = @6
                block  ;; label = @7
                  i32.const 0
                  i32.load offset=1049296
                  local.get 3
                  i32.const 2
                  i32.shl
                  i32.add
                  local.tee 0
                  i32.load
                  local.tee 6
                  br_if 0 (;@7;)
                  local.get 2
                  local.get 0
                  i32.store offset=4
                  local.get 2
                  i32.const 0
                  i32.store
                  i32.const 8
                  i32.const 12
                  local.get 3
                  i32.const 32
                  i32.lt_u
                  select
                  i32.const 1049280
                  i32.add
                  local.tee 6
                  local.get 6
                  i32.load
                  i32.const 1
                  local.get 3
                  i32.shl
                  i32.xor
                  i32.store
                  br 1 (;@6;)
                end
                local.get 2
                local.get 0
                i32.store offset=4
                local.get 2
                local.get 6
                i32.store
                local.get 0
                local.get 2
                i32.store
                local.get 6
                i32.const 4
                i32.add
                local.set 0
              end
              local.get 0
              local.get 2
              i32.store
              local.get 2
              local.get 1
              i32.store offset=8
              local.get 4
              i32.const -4
              i32.add
              local.get 1
              i32.store
              local.get 7
              local.set 0
              br 4 (;@1;)
            end
            local.get 6
            i32.load offset=8
            local.tee 3
            local.get 6
            i32.load offset=4
            local.tee 1
            i32.store
            block  ;; label = @5
              local.get 1
              i32.eqz
              br_if 0 (;@5;)
              local.get 1
              local.get 3
              i32.store offset=4
            end
            block  ;; label = @5
              i32.const 0
              i32.load offset=1049296
              local.tee 1
              local.get 2
              i32.const 2
              i32.shl
              i32.add
              i32.load
              br_if 0 (;@5;)
              i32.const 8
              i32.const 12
              local.get 2
              i32.const 32
              i32.lt_u
              select
              i32.const 1049280
              i32.add
              local.tee 3
              local.get 3
              i32.load
              i32.const 1
              local.get 2
              i32.shl
              i32.xor
              i32.store
            end
            local.get 5
            i32.const -4
            i32.and
            local.set 3
            block  ;; label = @5
              local.get 7
              local.get 4
              i32.sub
              i32.const 11
              i32.gt_u
              br_if 0 (;@5;)
              local.get 7
              local.get 3
              i32.const 1
              i32.add
              i32.store
              local.get 4
              local.get 7
              i32.eq
              br_if 4 (;@1;)
              local.get 4
              local.get 7
              i32.store
              local.get 0
              return
            end
            block  ;; label = @5
              block  ;; label = @6
                local.get 7
                local.get 4
                i32.const 4
                i32.add
                local.tee 5
                i32.sub
                i32.const 4
                i32.add
                local.tee 2
                i32.const 64
                i32.lt_u
                br_if 0 (;@6;)
                block  ;; label = @7
                  local.get 2
                  i32.const 128
                  i32.ge_u
                  br_if 0 (;@7;)
                  local.get 2
                  i32.const 3
                  i32.shr_u
                  i32.const 5
                  i32.add
                  local.set 6
                  br 2 (;@5;)
                end
                local.get 2
                i32.const 30
                local.get 2
                i32.clz
                local.tee 6
                i32.sub
                i32.shr_u
                local.get 6
                i32.const 1
                i32.shl
                i32.sub
                i32.const 67
                i32.add
                local.tee 6
                i32.const 63
                local.get 6
                i32.const 63
                i32.lt_u
                select
                local.set 6
                br 1 (;@5;)
              end
              local.get 2
              i32.const -12
              i32.add
              i32.const 2
              i32.shr_u
              local.set 6
            end
            block  ;; label = @5
              block  ;; label = @6
                local.get 1
                local.get 6
                i32.const 2
                i32.shl
                i32.add
                local.tee 1
                i32.load
                local.tee 8
                br_if 0 (;@6;)
                local.get 4
                local.get 1
                i32.store offset=8
                local.get 4
                i32.const 0
                i32.store offset=4
                i32.const 8
                i32.const 12
                local.get 6
                i32.const 32
                i32.lt_u
                select
                i32.const 1049280
                i32.add
                local.tee 8
                local.get 8
                i32.load
                i32.const 1
                local.get 6
                i32.shl
                i32.xor
                i32.store
                br 1 (;@5;)
              end
              local.get 4
              local.get 1
              i32.store offset=8
              local.get 4
              local.get 8
              i32.store offset=4
              local.get 1
              local.get 5
              i32.store
              local.get 8
              i32.const 4
              i32.add
              local.set 1
            end
            local.get 1
            local.get 5
            i32.store
            local.get 4
            local.get 2
            i32.store offset=12
            local.get 7
            local.get 2
            i32.store
            local.get 4
            local.get 3
            i32.const 3
            i32.add
            i32.store
            local.get 0
            return
          end
          local.get 1
          i32.load offset=8
          local.tee 9
          local.get 1
          i32.load offset=4
          local.tee 1
          i32.store
          block  ;; label = @4
            local.get 1
            i32.eqz
            br_if 0 (;@4;)
            local.get 1
            local.get 9
            i32.store offset=4
          end
          block  ;; label = @4
            i32.const 0
            i32.load offset=1049296
            local.get 8
            i32.const 2
            i32.shl
            i32.add
            i32.load
            br_if 0 (;@4;)
            i32.const 8
            i32.const 12
            local.get 8
            i32.const 32
            i32.lt_u
            select
            i32.const 1049280
            i32.add
            local.tee 1
            local.get 1
            i32.load
            i32.const 1
            local.get 8
            i32.shl
            i32.xor
            i32.store
          end
          local.get 6
          local.get 4
          i32.add
          local.set 6
        end
        block  ;; label = @3
          block  ;; label = @4
            local.get 6
            local.get 7
            i32.sub
            local.tee 1
            i32.const 64
            i32.lt_u
            br_if 0 (;@4;)
            block  ;; label = @5
              local.get 1
              i32.const 128
              i32.ge_u
              br_if 0 (;@5;)
              local.get 1
              i32.const 3
              i32.shr_u
              i32.const 5
              i32.add
              local.set 8
              br 2 (;@3;)
            end
            local.get 1
            i32.const 30
            local.get 1
            i32.clz
            local.tee 4
            i32.sub
            i32.shr_u
            local.get 4
            i32.const 1
            i32.shl
            i32.sub
            i32.const 67
            i32.add
            local.tee 4
            i32.const 63
            local.get 4
            i32.const 63
            i32.lt_u
            select
            local.set 8
            br 1 (;@3;)
          end
          local.get 1
          i32.const -12
          i32.add
          i32.const 2
          i32.shr_u
          local.set 8
        end
        block  ;; label = @3
          block  ;; label = @4
            i32.const 0
            i32.load offset=1049296
            local.get 8
            i32.const 2
            i32.shl
            i32.add
            local.tee 4
            i32.load
            local.tee 9
            br_if 0 (;@4;)
            local.get 2
            local.get 4
            i32.store offset=8
            local.get 2
            i32.const 0
            i32.store offset=4
            i32.const 8
            i32.const 12
            local.get 8
            i32.const 32
            i32.lt_u
            select
            i32.const 1049280
            i32.add
            local.tee 9
            local.get 9
            i32.load
            i32.const 1
            local.get 8
            i32.shl
            i32.xor
            i32.store
            br 1 (;@3;)
          end
          local.get 2
          local.get 4
          i32.store offset=8
          local.get 2
          local.get 9
          i32.store offset=4
          local.get 4
          local.get 7
          i32.store
          local.get 9
          i32.const 4
          i32.add
          local.set 4
        end
        local.get 4
        local.get 7
        i32.store
        local.get 2
        local.get 1
        i32.store offset=12
        local.get 6
        i32.const -4
        i32.add
        local.get 1
        i32.store
        local.get 2
        local.get 5
        i32.const 3
        i32.add
        i32.store
      end
      local.get 2
      local.get 3
      i32.eq
      br_if 0 (;@1;)
      local.get 3
      local.get 2
      i32.store
      local.get 0
      return
    end
    local.get 0)
  (func $_ZN4talc4talc13Talc$LT$O$GT$6malloc17h3aea4463430a8bb9E (type 2) (param i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    i32.const 12
    local.get 0
    i32.const 7
    i32.add
    i32.const -4
    i32.and
    local.get 0
    i32.const 9
    i32.lt_u
    select
    local.tee 2
    i32.const 3
    i32.shr_u
    i32.const 5
    i32.add
    local.get 2
    i32.const -12
    i32.add
    i32.const 2
    i32.shr_u
    local.get 2
    i32.const 30
    local.get 2
    i32.clz
    local.tee 3
    i32.sub
    i32.shr_u
    local.get 3
    i32.const 1
    i32.shl
    i32.sub
    i32.const 67
    i32.add
    local.tee 3
    i32.const 63
    local.get 3
    i32.const 63
    i32.lt_u
    select
    local.get 2
    i32.const 64
    i32.lt_u
    select
    local.tee 4
    local.get 2
    i32.const -64
    i32.and
    i32.const 64
    i32.eq
    local.tee 3
    select
    local.set 5
    local.get 4
    i32.const 31
    i32.and
    local.set 6
    local.get 3
    local.get 4
    i32.const 32
    i32.lt_u
    i32.or
    local.set 7
    local.get 0
    i32.const 65543
    i32.add
    i32.const 16
    i32.shr_u
    local.set 8
    block  ;; label = @1
      block  ;; label = @2
        loop  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 7
                br_if 0 (;@6;)
                local.get 4
                i32.const 63
                i32.gt_u
                br_if 2 (;@4;)
                i32.const 0
                i32.load offset=1049292
                local.tee 9
                local.get 6
                i32.shr_u
                local.tee 3
                i32.eqz
                br_if 2 (;@4;)
                local.get 3
                i32.ctz
                local.get 4
                i32.add
                local.set 10
                i32.const 0
                i32.load offset=1049288
                local.set 11
                br 1 (;@5;)
              end
              block  ;; label = @6
                i32.const 0
                i32.load offset=1049288
                local.tee 11
                local.get 5
                i32.shr_u
                local.tee 3
                i32.eqz
                br_if 0 (;@6;)
                local.get 3
                i32.ctz
                local.get 5
                i32.add
                local.set 10
                i32.const 0
                i32.load offset=1049292
                local.set 9
                br 1 (;@5;)
              end
              i32.const 0
              i32.load offset=1049292
              local.tee 9
              i32.eqz
              br_if 1 (;@4;)
              local.get 9
              i32.ctz
              i32.const 32
              i32.or
              local.set 10
            end
            i32.const 0
            i32.load offset=1049296
            local.set 12
            block  ;; label = @5
              local.get 9
              i32.eqz
              br_if 0 (;@5;)
              local.get 9
              i32.ctz
              i32.const 32
              i32.or
              local.set 13
              loop  ;; label = @6
                local.get 12
                local.get 10
                i32.const 2
                i32.shl
                i32.add
                local.set 3
                block  ;; label = @7
                  loop  ;; label = @8
                    local.get 3
                    i32.load
                    local.tee 3
                    i32.eqz
                    br_if 1 (;@7;)
                    local.get 3
                    i32.load offset=8
                    local.tee 14
                    local.get 2
                    i32.lt_u
                    br_if 0 (;@8;)
                    br 6 (;@2;)
                  end
                end
                local.get 10
                i32.const 1
                i32.add
                local.set 3
                block  ;; label = @7
                  local.get 10
                  i32.const 31
                  i32.lt_u
                  br_if 0 (;@7;)
                  local.get 10
                  i32.const 62
                  i32.gt_u
                  br_if 3 (;@4;)
                  local.get 9
                  local.get 3
                  i32.shr_u
                  local.tee 14
                  i32.eqz
                  br_if 3 (;@4;)
                  local.get 14
                  i32.ctz
                  local.get 3
                  i32.add
                  local.set 10
                  br 1 (;@6;)
                end
                local.get 13
                local.set 10
                local.get 11
                local.get 3
                i32.shr_u
                local.tee 14
                i32.eqz
                br_if 0 (;@6;)
                local.get 14
                i32.ctz
                local.get 3
                i32.add
                local.set 10
                br 0 (;@6;)
              end
            end
            loop  ;; label = @5
              local.get 12
              local.get 10
              i32.const 2
              i32.shl
              i32.add
              local.set 3
              block  ;; label = @6
                loop  ;; label = @7
                  local.get 3
                  i32.load
                  local.tee 3
                  i32.eqz
                  br_if 1 (;@6;)
                  local.get 3
                  i32.load offset=8
                  local.tee 14
                  local.get 2
                  i32.ge_u
                  br_if 5 (;@2;)
                  br 0 (;@7;)
                end
              end
              local.get 10
              i32.const 30
              i32.gt_u
              br_if 1 (;@4;)
              local.get 11
              local.get 10
              i32.const 1
              i32.add
              local.tee 3
              i32.shr_u
              local.tee 14
              i32.eqz
              br_if 1 (;@4;)
              local.get 14
              i32.ctz
              local.get 3
              i32.add
              local.set 10
              br 0 (;@5;)
            end
          end
          local.get 8
          local.set 3
          block  ;; label = @4
            loop  ;; label = @5
              local.get 3
              memory.grow
              local.tee 14
              i32.const -1
              i32.ne
              br_if 1 (;@4;)
              local.get 3
              i32.const 1
              i32.gt_u
              local.set 14
              local.get 3
              i32.const 1
              i32.shr_u
              local.set 3
              local.get 14
              br_if 0 (;@5;)
            end
            i32.const 0
            local.set 3
            br 3 (;@1;)
          end
          local.get 14
          i32.const 16
          i32.shl
          local.tee 14
          local.get 3
          i32.const 16
          i32.shl
          local.tee 12
          i32.add
          local.set 3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                block  ;; label = @15
                                  block  ;; label = @16
                                    i32.const 0
                                    i32.load offset=1049284
                                    local.tee 9
                                    i32.const 0
                                    i32.load offset=1049280
                                    local.tee 10
                                    i32.le_u
                                    br_if 0 (;@16;)
                                    local.get 9
                                    local.get 14
                                    i32.ne
                                    br_if 0 (;@16;)
                                    i32.const 0
                                    i32.load offset=1049296
                                    local.tee 12
                                    i32.eqz
                                    br_if 1 (;@15;)
                                    local.get 14
                                    local.get 10
                                    i32.sub
                                    i32.const 15
                                    i32.le_u
                                    br_if 2 (;@14;)
                                    local.get 3
                                    local.get 14
                                    i32.lt_u
                                    br_if 3 (;@13;)
                                    local.get 10
                                    i32.eqz
                                    br_if 4 (;@12;)
                                    local.get 10
                                    i32.const -5
                                    i32.gt_u
                                    br_if 5 (;@11;)
                                    local.get 10
                                    i32.const 3
                                    i32.add
                                    i32.const -4
                                    i32.and
                                    local.tee 10
                                    local.get 14
                                    i32.ge_u
                                    br_if 5 (;@11;)
                                    block  ;; label = @17
                                      local.get 9
                                      i32.const -4
                                      i32.add
                                      local.tee 13
                                      i32.load
                                      local.tee 11
                                      i32.const 1
                                      i32.and
                                      br_if 0 (;@17;)
                                      local.get 9
                                      local.get 11
                                      i32.sub
                                      local.set 14
                                      local.get 11
                                      i32.const 64
                                      i32.lt_u
                                      br_if 7 (;@10;)
                                      block  ;; label = @18
                                        local.get 11
                                        i32.const 128
                                        i32.ge_u
                                        br_if 0 (;@18;)
                                        local.get 11
                                        i32.const 3
                                        i32.shr_u
                                        i32.const 5
                                        i32.add
                                        local.set 9
                                        br 12 (;@6;)
                                      end
                                      local.get 11
                                      i32.const 30
                                      local.get 11
                                      i32.clz
                                      local.tee 9
                                      i32.sub
                                      i32.shr_u
                                      local.get 9
                                      i32.const 1
                                      i32.shl
                                      i32.sub
                                      i32.const 67
                                      i32.add
                                      local.tee 9
                                      i32.const 63
                                      local.get 9
                                      i32.const 63
                                      i32.lt_u
                                      select
                                      local.set 9
                                      br 11 (;@6;)
                                    end
                                    block  ;; label = @17
                                      local.get 3
                                      local.get 14
                                      i32.ne
                                      br_if 0 (;@17;)
                                      local.get 9
                                      local.set 3
                                      br 12 (;@5;)
                                    end
                                    block  ;; label = @17
                                      block  ;; label = @18
                                        local.get 12
                                        local.get 3
                                        local.get 14
                                        i32.sub
                                        local.tee 11
                                        i32.const 30
                                        local.get 11
                                        i32.clz
                                        local.tee 14
                                        i32.sub
                                        i32.shr_u
                                        local.get 14
                                        i32.const 1
                                        i32.shl
                                        i32.sub
                                        i32.const 67
                                        i32.add
                                        local.tee 15
                                        i32.const 63
                                        local.get 15
                                        i32.const 63
                                        i32.lt_u
                                        select
                                        local.tee 16
                                        i32.const 2
                                        i32.shl
                                        i32.add
                                        local.tee 14
                                        i32.load
                                        local.tee 17
                                        br_if 0 (;@18;)
                                        local.get 9
                                        local.get 14
                                        i32.store offset=4
                                        local.get 9
                                        i32.const 0
                                        i32.store
                                        i32.const 8
                                        i32.const 12
                                        local.get 15
                                        i32.const 32
                                        i32.lt_u
                                        select
                                        i32.const 1049280
                                        i32.add
                                        local.tee 15
                                        local.get 15
                                        i32.load
                                        i32.const 1
                                        local.get 16
                                        i32.shl
                                        i32.xor
                                        i32.store
                                        br 1 (;@17;)
                                      end
                                      local.get 9
                                      local.get 14
                                      i32.store offset=4
                                      local.get 9
                                      local.get 17
                                      i32.store
                                      local.get 14
                                      local.get 9
                                      i32.store
                                      local.get 17
                                      i32.const 4
                                      i32.add
                                      local.set 14
                                    end
                                    local.get 14
                                    local.get 9
                                    i32.store
                                    local.get 9
                                    local.get 11
                                    i32.store offset=8
                                    local.get 3
                                    i32.const -4
                                    i32.add
                                    local.get 11
                                    i32.store
                                    local.get 13
                                    local.get 13
                                    i32.load
                                    i32.const 2
                                    i32.add
                                    i32.store
                                    br 11 (;@5;)
                                  end
                                  block  ;; label = @16
                                    local.get 14
                                    br_if 0 (;@16;)
                                    local.get 3
                                    br_if 7 (;@9;)
                                  end
                                  block  ;; label = @16
                                    local.get 3
                                    local.get 14
                                    i32.le_u
                                    br_if 0 (;@16;)
                                    block  ;; label = @17
                                      i32.const 0
                                      i32.load offset=1049296
                                      local.tee 10
                                      br_if 0 (;@17;)
                                      local.get 14
                                      local.get 3
                                      i32.eq
                                      br_if 1 (;@16;)
                                      local.get 14
                                      i32.const 1
                                      i32.store
                                      i32.const 0
                                      local.get 14
                                      i32.const 4
                                      i32.add
                                      i32.const 0
                                      i32.const 256
                                      call $memset
                                      local.tee 11
                                      i32.store offset=1049296
                                      local.get 14
                                      i32.const 264
                                      i32.add
                                      local.set 9
                                      block  ;; label = @18
                                        block  ;; label = @19
                                          local.get 11
                                          local.get 12
                                          i32.const -264
                                          i32.add
                                          local.tee 10
                                          i32.const 30
                                          local.get 10
                                          i32.clz
                                          local.tee 12
                                          i32.sub
                                          i32.shr_u
                                          local.get 12
                                          i32.const 1
                                          i32.shl
                                          i32.sub
                                          i32.const 67
                                          i32.add
                                          local.tee 13
                                          i32.const 63
                                          local.get 13
                                          i32.const 63
                                          i32.lt_u
                                          select
                                          local.tee 15
                                          i32.const 2
                                          i32.shl
                                          i32.add
                                          local.tee 12
                                          i32.load
                                          local.tee 11
                                          br_if 0 (;@19;)
                                          i32.const 8
                                          i32.const 12
                                          local.get 13
                                          i32.const 32
                                          i32.lt_u
                                          select
                                          i32.const 1049280
                                          i32.add
                                          local.tee 13
                                          local.get 13
                                          i32.load
                                          i32.const 1
                                          local.get 15
                                          i32.shl
                                          i32.xor
                                          i32.store
                                          local.get 12
                                          local.set 13
                                          br 1 (;@18;)
                                        end
                                        local.get 12
                                        local.get 9
                                        i32.store
                                        local.get 11
                                        i32.const 4
                                        i32.add
                                        local.set 13
                                      end
                                      local.get 14
                                      local.get 12
                                      i32.store offset=268
                                      local.get 9
                                      local.get 11
                                      i32.store
                                      local.get 13
                                      local.get 9
                                      i32.store
                                      local.get 14
                                      local.get 10
                                      i32.store offset=272
                                      local.get 3
                                      i32.const -4
                                      i32.add
                                      local.get 10
                                      i32.store
                                      local.get 14
                                      i32.const 260
                                      i32.add
                                      local.get 14
                                      i32.const 3
                                      i32.add
                                      i32.store
                                      br 10 (;@7;)
                                    end
                                    local.get 14
                                    local.get 3
                                    i32.ne
                                    br_if 8 (;@8;)
                                  end
                                  local.get 1
                                  i32.const 31
                                  i32.add
                                  call $_ZN4core6result13unwrap_failed17h4a2f9aa3e457659bE
                                  unreachable
                                end
                                i32.const 1049024
                                i32.const 38
                                i32.const 1049064
                                call $_ZN4core9panicking5panic17hb6f6f50ca2c94e95E
                                unreachable
                              end
                              i32.const 1049096
                              i32.const 50
                              i32.const 1049148
                              call $_ZN4core9panicking5panic17hb6f6f50ca2c94e95E
                              unreachable
                            end
                            local.get 1
                            i32.const 4
                            i32.add
                            i32.const 1049080
                            call $_ZN4core9panicking9panic_fmt17ha937d72d43ea537bE
                            unreachable
                          end
                          local.get 1
                          i32.const 4
                          i32.add
                          i32.const 1049180
                          call $_ZN4core9panicking9panic_fmt17ha937d72d43ea537bE
                          unreachable
                        end
                        i32.const 1049164
                        call $_ZN4core6option13unwrap_failed17h2f8efb107ba0f1abE
                        unreachable
                      end
                      local.get 11
                      i32.const -12
                      i32.add
                      i32.const 2
                      i32.shr_u
                      local.set 9
                      br 3 (;@6;)
                    end
                    local.get 1
                    i32.const 4
                    i32.add
                    i32.const 1049008
                    call $_ZN4core9panicking9panic_fmt17ha937d72d43ea537bE
                    unreachable
                  end
                  local.get 14
                  i32.const 3
                  i32.store
                  local.get 14
                  i32.const 4
                  i32.add
                  local.set 11
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 10
                      local.get 12
                      i32.const -4
                      i32.add
                      local.tee 9
                      i32.const 30
                      local.get 9
                      i32.clz
                      local.tee 12
                      i32.sub
                      i32.shr_u
                      local.get 12
                      i32.const 1
                      i32.shl
                      i32.sub
                      i32.const 67
                      i32.add
                      local.tee 12
                      i32.const 63
                      local.get 12
                      i32.const 63
                      i32.lt_u
                      select
                      local.tee 15
                      i32.const 2
                      i32.shl
                      i32.add
                      local.tee 10
                      i32.load
                      local.tee 13
                      br_if 0 (;@9;)
                      local.get 14
                      i32.const 8
                      i32.add
                      local.get 10
                      i32.store
                      local.get 11
                      i32.const 0
                      i32.store
                      i32.const 8
                      i32.const 12
                      local.get 12
                      i32.const 32
                      i32.lt_u
                      select
                      i32.const 1049280
                      i32.add
                      local.tee 12
                      local.get 12
                      i32.load
                      i32.const 1
                      local.get 15
                      i32.shl
                      i32.xor
                      i32.store
                      br 1 (;@8;)
                    end
                    local.get 14
                    i32.const 8
                    i32.add
                    local.get 10
                    i32.store
                    local.get 11
                    local.get 13
                    i32.store
                    local.get 10
                    local.get 11
                    i32.store
                    local.get 13
                    i32.const 4
                    i32.add
                    local.set 10
                  end
                  local.get 10
                  local.get 11
                  i32.store
                  local.get 14
                  i32.const 12
                  i32.add
                  local.get 9
                  i32.store
                  local.get 3
                  i32.const -4
                  i32.add
                  local.get 9
                  i32.store
                end
                local.get 14
                local.set 10
                br 2 (;@4;)
              end
              local.get 14
              i32.load offset=4
              local.tee 13
              local.get 14
              i32.load
              local.tee 11
              i32.store
              block  ;; label = @6
                local.get 11
                i32.eqz
                br_if 0 (;@6;)
                local.get 11
                local.get 13
                i32.store offset=4
              end
              block  ;; label = @6
                local.get 12
                local.get 9
                i32.const 2
                i32.shl
                i32.add
                i32.load
                br_if 0 (;@6;)
                i32.const 8
                i32.const 12
                local.get 9
                i32.const 32
                i32.lt_u
                select
                i32.const 1049280
                i32.add
                local.tee 11
                local.get 11
                i32.load
                i32.const 1
                local.get 9
                i32.shl
                i32.xor
                i32.store
              end
              block  ;; label = @6
                block  ;; label = @7
                  local.get 3
                  local.get 14
                  i32.sub
                  local.tee 9
                  i32.const 64
                  i32.lt_u
                  br_if 0 (;@7;)
                  block  ;; label = @8
                    local.get 9
                    i32.const 128
                    i32.ge_u
                    br_if 0 (;@8;)
                    local.get 9
                    i32.const 3
                    i32.shr_u
                    i32.const 5
                    i32.add
                    local.set 13
                    br 2 (;@6;)
                  end
                  local.get 9
                  i32.const 30
                  local.get 9
                  i32.clz
                  local.tee 11
                  i32.sub
                  i32.shr_u
                  local.get 11
                  i32.const 1
                  i32.shl
                  i32.sub
                  i32.const 67
                  i32.add
                  local.tee 11
                  i32.const 63
                  local.get 11
                  i32.const 63
                  i32.lt_u
                  select
                  local.set 13
                  br 1 (;@6;)
                end
                local.get 9
                i32.const -12
                i32.add
                i32.const 2
                i32.shr_u
                local.set 13
              end
              block  ;; label = @6
                block  ;; label = @7
                  local.get 12
                  local.get 13
                  i32.const 2
                  i32.shl
                  i32.add
                  local.tee 11
                  i32.load
                  local.tee 15
                  br_if 0 (;@7;)
                  local.get 14
                  local.get 11
                  i32.store offset=4
                  local.get 14
                  i32.const 0
                  i32.store
                  i32.const 8
                  i32.const 12
                  local.get 13
                  i32.const 32
                  i32.lt_u
                  select
                  i32.const 1049280
                  i32.add
                  local.tee 15
                  local.get 15
                  i32.load
                  i32.const 1
                  local.get 13
                  i32.shl
                  i32.xor
                  i32.store
                  br 1 (;@6;)
                end
                local.get 14
                local.get 11
                i32.store offset=4
                local.get 14
                local.get 15
                i32.store
                local.get 11
                local.get 14
                i32.store
                local.get 15
                i32.const 4
                i32.add
                local.set 11
              end
              local.get 11
              local.get 14
              i32.store
              local.get 14
              local.get 9
              i32.store offset=8
              local.get 3
              i32.const -4
              i32.add
              local.get 9
              i32.store
            end
            local.get 10
            i32.load8_u
            i32.const 2
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            block  ;; label = @5
              block  ;; label = @6
                local.get 10
                i32.load offset=12
                local.tee 14
                i32.const 64
                i32.lt_u
                local.tee 13
                br_if 0 (;@6;)
                block  ;; label = @7
                  local.get 14
                  i32.const 128
                  i32.ge_u
                  br_if 0 (;@7;)
                  local.get 14
                  i32.const 3
                  i32.shr_u
                  i32.const 5
                  i32.add
                  local.set 9
                  br 2 (;@5;)
                end
                local.get 14
                i32.const 30
                local.get 14
                i32.clz
                local.tee 9
                i32.sub
                i32.shr_u
                local.get 9
                i32.const 1
                i32.shl
                i32.sub
                i32.const 67
                i32.add
                local.tee 9
                i32.const 63
                local.get 9
                i32.const 63
                i32.lt_u
                select
                local.set 9
                br 1 (;@5;)
              end
              local.get 14
              i32.const -12
              i32.add
              i32.const 2
              i32.shr_u
              local.set 9
            end
            local.get 10
            i32.load offset=8
            local.tee 15
            local.get 10
            i32.load offset=4
            local.tee 11
            i32.store
            block  ;; label = @5
              local.get 11
              i32.eqz
              br_if 0 (;@5;)
              local.get 11
              local.get 15
              i32.store offset=4
            end
            block  ;; label = @5
              local.get 12
              local.get 9
              i32.const 2
              i32.shl
              i32.add
              i32.load
              br_if 0 (;@5;)
              i32.const 8
              i32.const 12
              local.get 9
              i32.const 32
              i32.lt_u
              select
              i32.const 1049280
              i32.add
              local.tee 11
              local.get 11
              i32.load
              i32.const 1
              local.get 9
              i32.shl
              i32.xor
              i32.store
            end
            local.get 10
            i32.const 4
            i32.add
            local.set 11
            block  ;; label = @5
              block  ;; label = @6
                local.get 13
                br_if 0 (;@6;)
                block  ;; label = @7
                  local.get 14
                  i32.const 128
                  i32.ge_u
                  br_if 0 (;@7;)
                  local.get 14
                  i32.const 3
                  i32.shr_u
                  i32.const 5
                  i32.add
                  local.set 13
                  br 2 (;@5;)
                end
                local.get 14
                i32.const 30
                local.get 14
                i32.clz
                local.tee 9
                i32.sub
                i32.shr_u
                local.get 9
                i32.const 1
                i32.shl
                i32.sub
                i32.const 67
                i32.add
                local.tee 9
                i32.const 63
                local.get 9
                i32.const 63
                i32.lt_u
                select
                local.set 13
                br 1 (;@5;)
              end
              local.get 14
              i32.const -12
              i32.add
              i32.const 2
              i32.shr_u
              local.set 13
            end
            local.get 11
            local.get 14
            i32.add
            local.set 15
            block  ;; label = @5
              block  ;; label = @6
                local.get 12
                local.get 13
                i32.const 2
                i32.shl
                i32.add
                local.tee 9
                i32.load
                local.tee 12
                br_if 0 (;@6;)
                local.get 10
                local.get 9
                i32.store offset=8
                local.get 10
                i32.const 0
                i32.store offset=4
                i32.const 8
                i32.const 12
                local.get 13
                i32.const 32
                i32.lt_u
                select
                i32.const 1049280
                i32.add
                local.tee 12
                local.get 12
                i32.load
                i32.const 1
                local.get 13
                i32.shl
                i32.xor
                i32.store
                br 1 (;@5;)
              end
              local.get 10
              local.get 9
              i32.store offset=8
              local.get 10
              local.get 12
              i32.store offset=4
              local.get 9
              local.get 11
              i32.store
              local.get 12
              i32.const 4
              i32.add
              local.set 9
            end
            local.get 9
            local.get 11
            i32.store
            local.get 10
            local.get 14
            i32.store offset=12
            local.get 15
            i32.const -4
            i32.add
            local.get 14
            i32.store
            local.get 10
            i32.const 3
            i32.store
          end
          i32.const 0
          local.get 3
          i32.store offset=1049284
          i32.const 0
          local.get 10
          i32.store offset=1049280
          br 0 (;@3;)
        end
      end
      local.get 3
      i32.load offset=4
      local.tee 4
      local.get 3
      i32.load
      local.tee 2
      i32.store
      block  ;; label = @2
        local.get 2
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        local.get 4
        i32.store offset=4
      end
      block  ;; label = @2
        i32.const 0
        i32.load offset=1049296
        local.tee 9
        local.get 10
        i32.const 2
        i32.shl
        i32.add
        i32.load
        br_if 0 (;@2;)
        i32.const 8
        i32.const 12
        local.get 10
        i32.const 32
        i32.lt_u
        select
        i32.const 1049280
        i32.add
        local.tee 2
        local.get 2
        i32.load
        i32.const 1
        local.get 10
        i32.shl
        i32.xor
        i32.store
      end
      block  ;; label = @2
        block  ;; label = @3
          local.get 3
          local.get 14
          i32.add
          local.tee 4
          i32.const -12
          i32.add
          local.tee 2
          local.get 3
          local.get 3
          local.get 2
          i32.gt_u
          select
          local.tee 14
          local.get 3
          i32.sub
          local.tee 2
          i32.const 11
          i32.gt_u
          br_if 0 (;@3;)
          local.get 3
          i32.const -4
          i32.add
          local.tee 2
          local.get 2
          i32.load
          i32.const -2
          i32.add
          i32.store
          local.get 3
          local.set 14
          br 1 (;@2;)
        end
        block  ;; label = @3
          block  ;; label = @4
            local.get 2
            i32.const 64
            i32.lt_u
            br_if 0 (;@4;)
            block  ;; label = @5
              local.get 2
              i32.const 128
              i32.ge_u
              br_if 0 (;@5;)
              local.get 2
              i32.const 3
              i32.shr_u
              i32.const 5
              i32.add
              local.set 7
              br 2 (;@3;)
            end
            local.get 2
            i32.const 30
            local.get 2
            i32.clz
            local.tee 10
            i32.sub
            i32.shr_u
            local.get 10
            i32.const 1
            i32.shl
            i32.sub
            i32.const 67
            i32.add
            local.tee 10
            i32.const 63
            local.get 10
            i32.const 63
            i32.lt_u
            select
            local.set 7
            br 1 (;@3;)
          end
          local.get 2
          i32.const -12
          i32.add
          i32.const 2
          i32.shr_u
          local.set 7
        end
        block  ;; label = @3
          block  ;; label = @4
            local.get 9
            local.get 7
            i32.const 2
            i32.shl
            i32.add
            local.tee 10
            i32.load
            local.tee 9
            br_if 0 (;@4;)
            local.get 3
            local.get 10
            i32.store offset=4
            local.get 3
            i32.const 0
            i32.store
            i32.const 8
            i32.const 12
            local.get 7
            i32.const 32
            i32.lt_u
            select
            i32.const 1049280
            i32.add
            local.tee 9
            local.get 9
            i32.load
            i32.const 1
            local.get 7
            i32.shl
            i32.xor
            i32.store
            br 1 (;@3;)
          end
          local.get 3
          local.get 10
          i32.store offset=4
          local.get 3
          local.get 9
          i32.store
          local.get 10
          local.get 3
          i32.store
          local.get 9
          i32.const 4
          i32.add
          local.set 10
        end
        local.get 10
        local.get 3
        i32.store
        local.get 3
        local.get 2
        i32.store offset=8
        local.get 14
        i32.const -4
        i32.add
        local.get 2
        i32.store
      end
      block  ;; label = @2
        block  ;; label = @3
          local.get 4
          local.get 14
          i32.const 8
          i32.add
          local.tee 2
          local.get 3
          local.get 0
          i32.add
          i32.const 3
          i32.add
          i32.const -4
          i32.and
          local.tee 10
          local.get 2
          local.get 10
          i32.gt_u
          select
          local.tee 2
          i32.const 4
          i32.add
          local.tee 7
          i32.sub
          local.tee 0
          i32.const 11
          i32.gt_u
          br_if 0 (;@3;)
          local.get 4
          i32.const -4
          i32.add
          local.set 2
          i32.const 1
          local.set 0
          br 1 (;@2;)
        end
        block  ;; label = @3
          block  ;; label = @4
            local.get 0
            i32.const 64
            i32.lt_u
            br_if 0 (;@4;)
            block  ;; label = @5
              local.get 0
              i32.const 128
              i32.ge_u
              br_if 0 (;@5;)
              local.get 0
              i32.const 3
              i32.shr_u
              i32.const 5
              i32.add
              local.set 12
              br 2 (;@3;)
            end
            local.get 0
            i32.const 30
            local.get 0
            i32.clz
            local.tee 9
            i32.sub
            i32.shr_u
            local.get 9
            i32.const 1
            i32.shl
            i32.sub
            i32.const 67
            i32.add
            local.tee 9
            i32.const 63
            local.get 9
            i32.const 63
            i32.lt_u
            select
            local.set 12
            br 1 (;@3;)
          end
          local.get 0
          i32.const -12
          i32.add
          i32.const 2
          i32.shr_u
          local.set 12
        end
        block  ;; label = @3
          block  ;; label = @4
            i32.const 0
            i32.load offset=1049296
            local.get 12
            i32.const 2
            i32.shl
            i32.add
            local.tee 9
            i32.load
            local.tee 5
            br_if 0 (;@4;)
            local.get 2
            local.get 9
            i32.store offset=8
            local.get 2
            i32.const 0
            i32.store offset=4
            i32.const 8
            i32.const 12
            local.get 12
            i32.const 32
            i32.lt_u
            select
            i32.const 1049280
            i32.add
            local.tee 5
            local.get 5
            i32.load
            i32.const 1
            local.get 12
            i32.shl
            i32.xor
            i32.store
            br 1 (;@3;)
          end
          local.get 2
          local.get 9
          i32.store offset=8
          local.get 2
          local.get 5
          i32.store offset=4
          local.get 9
          local.get 7
          i32.store
          local.get 5
          i32.const 4
          i32.add
          local.set 9
        end
        local.get 9
        local.get 7
        i32.store
        local.get 2
        local.get 0
        i32.store offset=12
        local.get 4
        i32.const -4
        i32.add
        local.get 0
        i32.store
        i32.const 3
        local.set 0
      end
      local.get 2
      local.get 14
      local.get 0
      i32.add
      i32.store
      local.get 2
      local.get 10
      i32.eq
      br_if 0 (;@1;)
      local.get 10
      local.get 2
      i32.store
    end
    local.get 1
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 3)
  (func $_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17ha5795937c2c1e100E (type 3) (param i32 i32)
    (local i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    i32.const 0
    local.set 3
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          i32.load
          local.tee 4
          i32.const 1
          i32.shl
          local.tee 5
          local.get 1
          local.get 5
          local.get 1
          i32.gt_u
          select
          local.tee 1
          i32.const 8
          local.get 1
          i32.const 8
          i32.gt_u
          select
          local.tee 1
          i32.const 0
          i32.ge_s
          br_if 0 (;@3;)
          br 1 (;@2;)
        end
        i32.const 0
        local.set 5
        block  ;; label = @3
          local.get 4
          i32.eqz
          br_if 0 (;@3;)
          local.get 2
          local.get 4
          i32.store offset=28
          local.get 2
          local.get 0
          i32.load offset=4
          i32.store offset=20
          i32.const 1
          local.set 5
        end
        local.get 2
        local.get 5
        i32.store offset=24
        local.get 2
        i32.const 8
        i32.add
        local.get 1
        local.get 2
        i32.const 20
        i32.add
        call $_ZN5alloc7raw_vec11finish_grow17h9c2a4df959a4cf6dE
        local.get 2
        i32.load offset=8
        i32.const 1
        i32.ne
        br_if 1 (;@1;)
        local.get 2
        i32.load offset=16
        local.set 0
        local.get 2
        i32.load offset=12
        local.set 3
      end
      local.get 3
      local.get 0
      i32.const 1048688
      call $_ZN5alloc7raw_vec12handle_error17ha45f451325e11ff9E
      unreachable
    end
    local.get 2
    i32.load offset=12
    local.set 4
    local.get 0
    local.get 1
    i32.store
    local.get 0
    local.get 4
    i32.store offset=4
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer)
  (func $_ZN5alloc7raw_vec12handle_error17ha45f451325e11ff9E (type 0) (param i32 i32 i32)
    block  ;; label = @1
      local.get 0
      br_if 0 (;@1;)
      local.get 2
      call $_ZN5alloc7raw_vec17capacity_overflow17h2dde86e9083a0852E
      unreachable
    end
    local.get 1
    call $_ZN5alloc5alloc18handle_alloc_error17hfa9c49f9a3e333ddE
    unreachable)
  (func $anyblox_decode (type 4) (param i32 i32 i32 i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i64 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 7
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        local.get 4
        i32.load8_u
        i32.eqz
        br_if 0 (;@2;)
        local.get 4
        i32.load offset=4
        local.set 8
        br 1 (;@1;)
      end
      i32.const 0
      local.set 8
      local.get 4
      i32.const 0
      i32.store offset=16
      local.get 4
      i64.const 4294967296
      i64.store offset=4 align=4
      local.get 4
      i32.const 1
      i32.store8
    end
    local.get 4
    i32.const 0
    i32.store offset=12
    local.get 4
    i32.const 4
    i32.add
    local.set 9
    block  ;; label = @1
      local.get 8
      local.get 3
      i32.ge_u
      br_if 0 (;@1;)
      local.get 9
      local.get 3
      call $_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$7reserve21do_reserve_and_handle17ha5795937c2c1e100E
    end
    local.get 4
    local.get 2
    i32.store offset=16
    block  ;; label = @1
      local.get 3
      local.get 2
      i32.add
      local.tee 10
      i32.eqz
      br_if 0 (;@1;)
      i32.const 0
      local.set 11
      i32.const 0
      local.set 12
      i32.const 0
      local.set 13
      i32.const 0
      local.set 8
      i32.const 0
      local.set 14
      i32.const 0
      local.set 15
      i32.const 0
      local.set 16
      loop  ;; label = @2
        local.get 10
        local.get 16
        i32.sub
        local.set 3
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 15
                  br_if 0 (;@7;)
                  local.get 14
                  br_if 1 (;@6;)
                  local.get 13
                  i32.const 7
                  i32.add
                  i32.const 3
                  i32.shr_u
                  local.get 8
                  i32.add
                  local.tee 3
                  i32.const 1
                  i32.add
                  local.set 8
                  local.get 0
                  local.get 3
                  i32.add
                  i32.load8_s
                  local.tee 3
                  i32.const 127
                  i32.and
                  i64.extend_i32_u
                  local.set 17
                  block  ;; label = @8
                    local.get 3
                    i32.const -1
                    i32.gt_s
                    br_if 0 (;@8;)
                    i32.const 7
                    local.set 3
                    loop  ;; label = @9
                      local.get 0
                      local.get 8
                      i32.add
                      i32.load8_s
                      local.tee 2
                      i32.const 127
                      i32.and
                      i64.extend_i32_u
                      local.get 3
                      i32.const 63
                      i32.and
                      i64.extend_i32_u
                      i64.shl
                      local.get 17
                      i64.or
                      local.set 17
                      local.get 3
                      i32.const 7
                      i32.add
                      local.set 3
                      local.get 8
                      i32.const 1
                      i32.add
                      local.set 8
                      local.get 2
                      i32.const 0
                      i32.lt_s
                      br_if 0 (;@9;)
                    end
                  end
                  local.get 17
                  i64.const 1
                  i64.and
                  i64.eqz
                  br_if 3 (;@4;)
                  local.get 17
                  i32.wrap_i64
                  i32.const 2
                  i32.shl
                  i32.const -8
                  i32.and
                  local.set 14
                  i32.const 0
                  local.set 13
                  br 2 (;@5;)
                end
                local.get 3
                local.get 15
                local.get 3
                local.get 15
                i32.lt_u
                select
                local.set 18
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 10
                    local.get 16
                    i32.eq
                    br_if 0 (;@8;)
                    local.get 11
                    i32.const 1
                    i32.and
                    i32.eqz
                    br_if 1 (;@7;)
                    i32.const 70
                    i32.const 79
                    local.get 19
                    i32.const 255
                    i32.and
                    select
                    local.set 20
                    local.get 18
                    i32.const 1
                    local.get 18
                    i32.const 1
                    i32.gt_u
                    select
                    local.set 3
                    loop  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          local.get 4
                          i32.load offset=16
                          local.tee 2
                          br_if 0 (;@11;)
                          block  ;; label = @12
                            local.get 4
                            i32.load offset=12
                            local.tee 2
                            local.get 4
                            i32.load offset=4
                            i32.ne
                            br_if 0 (;@12;)
                            local.get 9
                            call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17h9bc7bed792d2c173E
                          end
                          local.get 4
                          i32.load offset=8
                          local.get 2
                          i32.add
                          local.get 20
                          i32.store8
                          local.get 4
                          local.get 2
                          i32.const 1
                          i32.add
                          i32.store offset=12
                          br 1 (;@10;)
                        end
                        local.get 4
                        local.get 2
                        i32.const -1
                        i32.add
                        i32.store offset=16
                      end
                      local.get 3
                      i32.const -1
                      i32.add
                      local.tee 3
                      br_if 0 (;@9;)
                    end
                  end
                  local.get 18
                  local.get 16
                  i32.add
                  local.set 16
                  local.get 15
                  local.get 18
                  i32.sub
                  local.set 15
                  br 4 (;@3;)
                end
                i32.const 1048736
                call $_ZN4core6option13unwrap_failed17h2f8efb107ba0f1abE
                unreachable
              end
              local.get 1
              local.get 8
              i32.sub
              i32.const 3
              i32.shl
              local.get 13
              i32.sub
              local.tee 2
              local.get 3
              local.get 14
              local.get 3
              local.get 14
              i32.lt_u
              select
              local.tee 3
              local.get 2
              local.get 3
              i32.lt_u
              select
              local.set 18
              i32.const 0
              local.set 20
              block  ;; label = @6
                block  ;; label = @7
                  local.get 13
                  br_if 0 (;@7;)
                  i32.const 0
                  local.set 2
                  local.get 8
                  local.set 3
                  br 1 (;@6;)
                end
                block  ;; label = @7
                  local.get 18
                  br_if 0 (;@7;)
                  local.get 13
                  local.set 2
                  local.get 8
                  local.set 3
                  br 1 (;@6;)
                end
                i32.const 1
                local.set 20
                local.get 8
                local.set 3
                loop  ;; label = @7
                  local.get 13
                  local.get 20
                  i32.add
                  local.tee 2
                  i32.const 32
                  i32.eq
                  local.set 15
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 4
                      i32.load offset=16
                      local.tee 14
                      br_if 0 (;@9;)
                      i32.const 70
                      i32.const 79
                      local.get 12
                      local.get 2
                      i32.const -1
                      i32.add
                      i32.shr_u
                      i32.const 1
                      i32.and
                      select
                      local.set 21
                      block  ;; label = @10
                        local.get 4
                        i32.load offset=12
                        local.tee 14
                        local.get 4
                        i32.load offset=4
                        i32.ne
                        br_if 0 (;@10;)
                        local.get 9
                        call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17h9bc7bed792d2c173E
                      end
                      local.get 4
                      i32.load offset=8
                      local.get 14
                      i32.add
                      local.get 21
                      i32.store8
                      local.get 4
                      local.get 14
                      i32.const 1
                      i32.add
                      i32.store offset=12
                      br 1 (;@8;)
                    end
                    local.get 4
                    local.get 14
                    i32.const -1
                    i32.add
                    i32.store offset=16
                  end
                  i32.const 0
                  local.get 2
                  local.get 15
                  select
                  local.set 2
                  local.get 3
                  i32.const 4
                  i32.add
                  local.tee 14
                  local.get 3
                  local.get 15
                  select
                  local.set 3
                  local.get 14
                  local.get 8
                  local.get 15
                  select
                  local.set 8
                  local.get 20
                  i32.const 1
                  i32.add
                  local.set 15
                  block  ;; label = @8
                    local.get 20
                    local.get 18
                    i32.ge_u
                    br_if 0 (;@8;)
                    local.get 15
                    local.set 20
                    local.get 2
                    br_if 1 (;@7;)
                  end
                end
                local.get 15
                i32.const -1
                i32.add
                local.set 20
              end
              block  ;; label = @6
                local.get 18
                local.get 20
                i32.sub
                i32.const 8
                i32.lt_u
                br_if 0 (;@6;)
                i32.const 0
                local.get 20
                i32.sub
                local.set 20
                loop  ;; label = @7
                  local.get 0
                  local.get 3
                  i32.add
                  i32.load8_s
                  local.set 8
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 4
                      i32.load offset=16
                      local.tee 13
                      br_if 0 (;@9;)
                      i32.const 70
                      i32.const 79
                      local.get 8
                      i32.const 1
                      i32.and
                      select
                      local.set 15
                      block  ;; label = @10
                        local.get 4
                        i32.load offset=12
                        local.tee 13
                        local.get 4
                        i32.load offset=4
                        i32.ne
                        br_if 0 (;@10;)
                        local.get 9
                        call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17h9bc7bed792d2c173E
                      end
                      local.get 4
                      i32.load offset=8
                      local.get 13
                      i32.add
                      local.get 15
                      i32.store8
                      local.get 4
                      local.get 13
                      i32.const 1
                      i32.add
                      i32.store offset=12
                      local.get 4
                      i32.load offset=16
                      local.set 13
                      br 1 (;@8;)
                    end
                    local.get 4
                    local.get 13
                    i32.const -1
                    i32.add
                    local.tee 13
                    i32.store offset=16
                  end
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 13
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 4
                      local.get 13
                      i32.const -1
                      i32.add
                      local.tee 13
                      i32.store offset=16
                      br 1 (;@8;)
                    end
                    i32.const 70
                    i32.const 79
                    local.get 8
                    i32.const 2
                    i32.and
                    select
                    local.set 15
                    block  ;; label = @9
                      local.get 4
                      i32.load offset=12
                      local.tee 13
                      local.get 4
                      i32.load offset=4
                      i32.ne
                      br_if 0 (;@9;)
                      local.get 9
                      call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17h9bc7bed792d2c173E
                    end
                    local.get 4
                    i32.load offset=8
                    local.get 13
                    i32.add
                    local.get 15
                    i32.store8
                    local.get 4
                    local.get 13
                    i32.const 1
                    i32.add
                    i32.store offset=12
                    local.get 4
                    i32.load offset=16
                    local.set 13
                  end
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 13
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 4
                      local.get 13
                      i32.const -1
                      i32.add
                      local.tee 13
                      i32.store offset=16
                      br 1 (;@8;)
                    end
                    i32.const 70
                    i32.const 79
                    local.get 8
                    i32.const 4
                    i32.and
                    select
                    local.set 15
                    block  ;; label = @9
                      local.get 4
                      i32.load offset=12
                      local.tee 13
                      local.get 4
                      i32.load offset=4
                      i32.ne
                      br_if 0 (;@9;)
                      local.get 9
                      call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17h9bc7bed792d2c173E
                    end
                    local.get 4
                    i32.load offset=8
                    local.get 13
                    i32.add
                    local.get 15
                    i32.store8
                    local.get 4
                    local.get 13
                    i32.const 1
                    i32.add
                    i32.store offset=12
                    local.get 4
                    i32.load offset=16
                    local.set 13
                  end
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 13
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 4
                      local.get 13
                      i32.const -1
                      i32.add
                      local.tee 13
                      i32.store offset=16
                      br 1 (;@8;)
                    end
                    i32.const 70
                    i32.const 79
                    local.get 8
                    i32.const 8
                    i32.and
                    select
                    local.set 15
                    block  ;; label = @9
                      local.get 4
                      i32.load offset=12
                      local.tee 13
                      local.get 4
                      i32.load offset=4
                      i32.ne
                      br_if 0 (;@9;)
                      local.get 9
                      call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17h9bc7bed792d2c173E
                    end
                    local.get 4
                    i32.load offset=8
                    local.get 13
                    i32.add
                    local.get 15
                    i32.store8
                    local.get 4
                    local.get 13
                    i32.const 1
                    i32.add
                    i32.store offset=12
                    local.get 4
                    i32.load offset=16
                    local.set 13
                  end
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 13
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 4
                      local.get 13
                      i32.const -1
                      i32.add
                      local.tee 13
                      i32.store offset=16
                      br 1 (;@8;)
                    end
                    i32.const 70
                    i32.const 79
                    local.get 8
                    i32.const 16
                    i32.and
                    select
                    local.set 15
                    block  ;; label = @9
                      local.get 4
                      i32.load offset=12
                      local.tee 13
                      local.get 4
                      i32.load offset=4
                      i32.ne
                      br_if 0 (;@9;)
                      local.get 9
                      call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17h9bc7bed792d2c173E
                    end
                    local.get 4
                    i32.load offset=8
                    local.get 13
                    i32.add
                    local.get 15
                    i32.store8
                    local.get 4
                    local.get 13
                    i32.const 1
                    i32.add
                    i32.store offset=12
                    local.get 4
                    i32.load offset=16
                    local.set 13
                  end
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 13
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 4
                      local.get 13
                      i32.const -1
                      i32.add
                      local.tee 13
                      i32.store offset=16
                      br 1 (;@8;)
                    end
                    i32.const 70
                    i32.const 79
                    local.get 8
                    i32.const 32
                    i32.and
                    select
                    local.set 15
                    block  ;; label = @9
                      local.get 4
                      i32.load offset=12
                      local.tee 13
                      local.get 4
                      i32.load offset=4
                      i32.ne
                      br_if 0 (;@9;)
                      local.get 9
                      call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17h9bc7bed792d2c173E
                    end
                    local.get 4
                    i32.load offset=8
                    local.get 13
                    i32.add
                    local.get 15
                    i32.store8
                    local.get 4
                    local.get 13
                    i32.const 1
                    i32.add
                    i32.store offset=12
                    local.get 4
                    i32.load offset=16
                    local.set 13
                  end
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 13
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 4
                      local.get 13
                      i32.const -1
                      i32.add
                      local.tee 13
                      i32.store offset=16
                      br 1 (;@8;)
                    end
                    i32.const 70
                    i32.const 79
                    local.get 8
                    i32.const 64
                    i32.and
                    select
                    local.set 15
                    block  ;; label = @9
                      local.get 4
                      i32.load offset=12
                      local.tee 13
                      local.get 4
                      i32.load offset=4
                      i32.ne
                      br_if 0 (;@9;)
                      local.get 9
                      call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17h9bc7bed792d2c173E
                    end
                    local.get 4
                    i32.load offset=8
                    local.get 13
                    i32.add
                    local.get 15
                    i32.store8
                    local.get 4
                    local.get 13
                    i32.const 1
                    i32.add
                    i32.store offset=12
                    local.get 4
                    i32.load offset=16
                    local.set 13
                  end
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 13
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 4
                      local.get 13
                      i32.const -1
                      i32.add
                      i32.store offset=16
                      br 1 (;@8;)
                    end
                    i32.const 79
                    i32.const 70
                    local.get 8
                    i32.const -1
                    i32.gt_s
                    select
                    local.set 13
                    block  ;; label = @9
                      local.get 4
                      i32.load offset=12
                      local.tee 8
                      local.get 4
                      i32.load offset=4
                      i32.ne
                      br_if 0 (;@9;)
                      local.get 9
                      call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17h9bc7bed792d2c173E
                    end
                    local.get 4
                    i32.load offset=8
                    local.get 8
                    i32.add
                    local.get 13
                    i32.store8
                    local.get 4
                    local.get 8
                    i32.const 1
                    i32.add
                    i32.store offset=12
                  end
                  local.get 3
                  i32.const 1
                  i32.add
                  local.set 3
                  local.get 18
                  local.get 20
                  i32.const -8
                  i32.add
                  local.tee 20
                  i32.add
                  i32.const 7
                  i32.gt_u
                  br_if 0 (;@7;)
                end
                i32.const 0
                local.get 20
                i32.sub
                local.set 20
                local.get 3
                local.set 8
              end
              block  ;; label = @6
                block  ;; label = @7
                  local.get 18
                  local.get 20
                  i32.gt_u
                  br_if 0 (;@7;)
                  local.get 2
                  local.set 13
                  br 1 (;@6;)
                end
                local.get 18
                local.get 20
                i32.sub
                local.set 20
                loop  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 2
                      br_if 0 (;@9;)
                      local.get 7
                      i32.const 0
                      i32.store offset=12
                      local.get 7
                      i32.const 12
                      i32.add
                      local.get 0
                      local.get 3
                      i32.add
                      local.get 1
                      local.get 3
                      i32.sub
                      local.tee 13
                      i32.const 4
                      local.get 13
                      i32.const 4
                      i32.lt_u
                      select
                      call $memcpy
                      drop
                      local.get 7
                      i32.load offset=12
                      local.set 12
                      i32.const 1
                      local.set 13
                      br 1 (;@8;)
                    end
                    local.get 2
                    i32.const 1
                    i32.add
                    local.tee 13
                    i32.const 32
                    i32.ne
                    br_if 0 (;@8;)
                    i32.const 0
                    local.set 13
                    local.get 3
                    i32.const 4
                    i32.add
                    local.tee 8
                    local.set 3
                  end
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 4
                      i32.load offset=16
                      local.tee 15
                      br_if 0 (;@9;)
                      i32.const 70
                      i32.const 79
                      local.get 12
                      local.get 2
                      i32.shr_u
                      i32.const 1
                      i32.and
                      select
                      local.set 15
                      block  ;; label = @10
                        local.get 4
                        i32.load offset=12
                        local.tee 2
                        local.get 4
                        i32.load offset=4
                        i32.ne
                        br_if 0 (;@10;)
                        local.get 9
                        call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17h9bc7bed792d2c173E
                      end
                      local.get 4
                      i32.load offset=8
                      local.get 2
                      i32.add
                      local.get 15
                      i32.store8
                      local.get 4
                      local.get 2
                      i32.const 1
                      i32.add
                      i32.store offset=12
                      br 1 (;@8;)
                    end
                    local.get 4
                    local.get 15
                    i32.const -1
                    i32.add
                    i32.store offset=16
                  end
                  local.get 13
                  local.set 2
                  local.get 20
                  i32.const -1
                  i32.add
                  local.tee 20
                  br_if 0 (;@7;)
                end
              end
              local.get 18
              local.get 16
              i32.add
              local.set 16
              i32.const 0
              local.set 14
            end
            i32.const 0
            local.set 15
            br 1 (;@3;)
          end
          i32.const 1
          local.set 11
          local.get 0
          local.get 8
          i32.add
          i32.load8_u
          local.set 19
          local.get 17
          i64.const 1
          i64.shr_u
          i32.wrap_i64
          local.set 15
          i32.const 0
          local.set 13
          local.get 8
          i32.const 1
          i32.add
          local.set 8
          i32.const 0
          local.set 14
        end
        local.get 16
        local.get 10
        i32.lt_u
        br_if 0 (;@2;)
      end
    end
    i32.const 0
    i32.const 1049236
    i32.store offset=1049268
    i32.const 0
    local.get 4
    i32.load offset=8
    i32.store offset=1049232
    local.get 4
    i32.load offset=12
    local.set 4
    i32.const 0
    i32.const 1049268
    i32.store offset=1049220
    i32.const 0
    i32.const 1049272
    i32.store offset=1049216
    i32.const 0
    i64.const 4294967297
    i64.store offset=1049208 align=4
    i32.const 0
    local.get 4
    i32.store offset=1049196
    i32.const 0
    i64.const 0
    i64.store offset=1049260 align=4
    i32.const 0
    i32.const 1049228
    i32.store offset=1049256
    i32.const 0
    i64.const 2
    i64.store offset=1049248 align=4
    i32.const 0
    local.get 4
    i32.store offset=1049236
    i32.const 0
    i32.const 0
    i32.store offset=1049200
    i32.const 0
    i32.const 0
    i32.store offset=1049240
    i32.const 0
    i32.const 0
    i32.store offset=1049224
    local.get 7
    i32.const 16
    i32.add
    global.set $__stack_pointer
    i32.const 1049196)
  (func $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$8grow_one17h9bc7bed792d2c173E (type 5) (param i32)
    (local i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          i32.load
          local.tee 2
          i32.const -1
          i32.ne
          br_if 0 (;@3;)
          i32.const 0
          local.set 3
          br 1 (;@2;)
        end
        i32.const 0
        local.set 3
        block  ;; label = @3
          local.get 2
          i32.const 1
          i32.shl
          local.tee 4
          local.get 2
          i32.const 1
          i32.add
          local.tee 5
          local.get 4
          local.get 5
          i32.gt_u
          select
          local.tee 4
          i32.const 8
          local.get 4
          i32.const 8
          i32.gt_u
          select
          local.tee 4
          i32.const 0
          i32.ge_s
          br_if 0 (;@3;)
          br 1 (;@2;)
        end
        i32.const 0
        local.set 5
        block  ;; label = @3
          local.get 2
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          local.get 2
          i32.store offset=28
          local.get 1
          local.get 0
          i32.load offset=4
          i32.store offset=20
          i32.const 1
          local.set 5
        end
        local.get 1
        local.get 5
        i32.store offset=24
        local.get 1
        i32.const 8
        i32.add
        local.get 4
        local.get 1
        i32.const 20
        i32.add
        call $_ZN5alloc7raw_vec11finish_grow17h1549f51b0029e56cE
        local.get 1
        i32.load offset=8
        i32.const 1
        i32.ne
        br_if 1 (;@1;)
        local.get 1
        i32.load offset=16
        local.set 0
        local.get 1
        i32.load offset=12
        local.set 3
      end
      local.get 3
      local.get 0
      i32.const 1048752
      call $_ZN5alloc7raw_vec12handle_error17ha45f451325e11ff9E
      unreachable
    end
    local.get 1
    i32.load offset=12
    local.set 2
    local.get 0
    local.get 4
    i32.store
    local.get 0
    local.get 2
    i32.store offset=4
    local.get 1
    i32.const 32
    i32.add
    global.set $__stack_pointer)
  (func $_ZN4core6option13unwrap_failed17h2f8efb107ba0f1abE (type 5) (param i32)
    i32.const 1048768
    i32.const 43
    local.get 0
    call $_ZN4core9panicking5panic17hb6f6f50ca2c94e95E
    unreachable)
  (func $__rust_alloc_error_handler (type 5) (param i32)
    local.get 0
    call $__rdl_oom
    unreachable)
  (func $__rdl_oom (type 5) (param i32)
    call $_ZN4core9panicking18panic_nounwind_fmt17h92ee45c5b5d6beadE
    unreachable)
  (func $_ZN5alloc7raw_vec11finish_grow17h1549f51b0029e56cE (type 0) (param i32 i32 i32)
    (local i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 2
          i32.load offset=4
          i32.eqz
          br_if 0 (;@3;)
          block  ;; label = @4
            local.get 2
            i32.load offset=8
            local.tee 3
            br_if 0 (;@4;)
            i32.const 0
            i32.load8_u offset=1049276
            drop
            br 2 (;@2;)
          end
          local.get 2
          i32.load
          local.get 3
          local.get 1
          call $__rust_realloc
          local.set 2
          br 2 (;@1;)
        end
        i32.const 0
        i32.load8_u offset=1049276
        drop
      end
      local.get 1
      call $_ZN4talc4talc13Talc$LT$O$GT$6malloc17h3aea4463430a8bb9E
      local.set 2
    end
    local.get 0
    local.get 1
    i32.store offset=8
    local.get 0
    local.get 2
    i32.const 1
    local.get 2
    select
    i32.store offset=4
    local.get 0
    local.get 2
    i32.eqz
    i32.store)
  (func $_ZN5alloc7raw_vec17capacity_overflow17h2dde86e9083a0852E (type 5) (param i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    local.get 1
    i32.const 8
    i32.add
    local.get 0
    call $_ZN4core9panicking9panic_fmt17ha937d72d43ea537bE
    unreachable)
  (func $_ZN5alloc5alloc18handle_alloc_error17hfa9c49f9a3e333ddE (type 5) (param i32)
    local.get 0
    call $__rust_alloc_error_handler
    unreachable)
  (func $_ZN4core9panicking9panic_fmt17ha937d72d43ea537bE (type 3) (param i32 i32)
    loop  ;; label = @1
      br 0 (;@1;)
    end)
  (func $_ZN4core9panicking18panic_nounwind_fmt17h92ee45c5b5d6beadE (type 6)
    loop  ;; label = @1
      br 0 (;@1;)
    end)
  (func $_ZN4core9panicking5panic17hb6f6f50ca2c94e95E (type 0) (param i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    i32.const 8
    i32.add
    local.get 2
    call $_ZN4core9panicking9panic_fmt17ha937d72d43ea537bE
    unreachable)
  (func $_ZN4core6result13unwrap_failed17h4a2f9aa3e457659bE (type 5) (param i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    local.get 1
    i32.const 8
    i32.add
    i32.const 1048904
    call $_ZN4core9panicking9panic_fmt17ha937d72d43ea537bE
    unreachable)
  (func $memset (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.const 16
        i32.ge_u
        br_if 0 (;@2;)
        local.get 0
        local.set 3
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 0
        i32.const 0
        local.get 0
        i32.sub
        i32.const 3
        i32.and
        local.tee 4
        i32.add
        local.tee 5
        local.get 0
        i32.le_u
        br_if 0 (;@2;)
        local.get 4
        i32.const -1
        i32.add
        local.set 6
        local.get 0
        local.set 3
        block  ;; label = @3
          local.get 4
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          local.set 7
          local.get 0
          local.set 3
          loop  ;; label = @4
            local.get 3
            local.get 1
            i32.store8
            local.get 3
            i32.const 1
            i32.add
            local.set 3
            local.get 7
            i32.const -1
            i32.add
            local.tee 7
            br_if 0 (;@4;)
          end
        end
        local.get 6
        i32.const 7
        i32.lt_u
        br_if 0 (;@2;)
        loop  ;; label = @3
          local.get 3
          local.get 1
          i32.store8
          local.get 3
          i32.const 7
          i32.add
          local.get 1
          i32.store8
          local.get 3
          i32.const 6
          i32.add
          local.get 1
          i32.store8
          local.get 3
          i32.const 5
          i32.add
          local.get 1
          i32.store8
          local.get 3
          i32.const 4
          i32.add
          local.get 1
          i32.store8
          local.get 3
          i32.const 3
          i32.add
          local.get 1
          i32.store8
          local.get 3
          i32.const 2
          i32.add
          local.get 1
          i32.store8
          local.get 3
          i32.const 1
          i32.add
          local.get 1
          i32.store8
          local.get 3
          i32.const 8
          i32.add
          local.tee 3
          local.get 5
          i32.ne
          br_if 0 (;@3;)
        end
      end
      block  ;; label = @2
        local.get 5
        local.get 5
        local.get 2
        local.get 4
        i32.sub
        local.tee 2
        i32.const -4
        i32.and
        i32.add
        local.tee 3
        i32.ge_u
        br_if 0 (;@2;)
        local.get 1
        i32.const 255
        i32.and
        i32.const 16843009
        i32.mul
        local.set 7
        loop  ;; label = @3
          local.get 5
          local.get 7
          i32.store
          local.get 5
          i32.const 4
          i32.add
          local.tee 5
          local.get 3
          i32.lt_u
          br_if 0 (;@3;)
        end
      end
      local.get 2
      i32.const 3
      i32.and
      local.set 2
    end
    block  ;; label = @1
      local.get 3
      local.get 3
      local.get 2
      i32.add
      local.tee 7
      i32.ge_u
      br_if 0 (;@1;)
      local.get 2
      i32.const -1
      i32.add
      local.set 4
      block  ;; label = @2
        local.get 2
        i32.const 7
        i32.and
        local.tee 5
        i32.eqz
        br_if 0 (;@2;)
        loop  ;; label = @3
          local.get 3
          local.get 1
          i32.store8
          local.get 3
          i32.const 1
          i32.add
          local.set 3
          local.get 5
          i32.const -1
          i32.add
          local.tee 5
          br_if 0 (;@3;)
        end
      end
      local.get 4
      i32.const 7
      i32.lt_u
      br_if 0 (;@1;)
      loop  ;; label = @2
        local.get 3
        local.get 1
        i32.store8
        local.get 3
        i32.const 7
        i32.add
        local.get 1
        i32.store8
        local.get 3
        i32.const 6
        i32.add
        local.get 1
        i32.store8
        local.get 3
        i32.const 5
        i32.add
        local.get 1
        i32.store8
        local.get 3
        i32.const 4
        i32.add
        local.get 1
        i32.store8
        local.get 3
        i32.const 3
        i32.add
        local.get 1
        i32.store8
        local.get 3
        i32.const 2
        i32.add
        local.get 1
        i32.store8
        local.get 3
        i32.const 1
        i32.add
        local.get 1
        i32.store8
        local.get 3
        i32.const 8
        i32.add
        local.tee 3
        local.get 7
        i32.ne
        br_if 0 (;@2;)
      end
    end
    local.get 0)
  (func $memcpy (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.const 16
        i32.ge_u
        br_if 0 (;@2;)
        local.get 0
        local.set 3
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 0
        i32.const 0
        local.get 0
        i32.sub
        i32.const 3
        i32.and
        local.tee 4
        i32.add
        local.tee 5
        local.get 0
        i32.le_u
        br_if 0 (;@2;)
        local.get 4
        i32.const -1
        i32.add
        local.set 6
        local.get 0
        local.set 3
        local.get 1
        local.set 7
        block  ;; label = @3
          local.get 4
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          local.set 8
          local.get 0
          local.set 3
          local.get 1
          local.set 7
          loop  ;; label = @4
            local.get 3
            local.get 7
            i32.load8_u
            i32.store8
            local.get 7
            i32.const 1
            i32.add
            local.set 7
            local.get 3
            i32.const 1
            i32.add
            local.set 3
            local.get 8
            i32.const -1
            i32.add
            local.tee 8
            br_if 0 (;@4;)
          end
        end
        local.get 6
        i32.const 7
        i32.lt_u
        br_if 0 (;@2;)
        loop  ;; label = @3
          local.get 3
          local.get 7
          i32.load8_u
          i32.store8
          local.get 3
          i32.const 1
          i32.add
          local.get 7
          i32.const 1
          i32.add
          i32.load8_u
          i32.store8
          local.get 3
          i32.const 2
          i32.add
          local.get 7
          i32.const 2
          i32.add
          i32.load8_u
          i32.store8
          local.get 3
          i32.const 3
          i32.add
          local.get 7
          i32.const 3
          i32.add
          i32.load8_u
          i32.store8
          local.get 3
          i32.const 4
          i32.add
          local.get 7
          i32.const 4
          i32.add
          i32.load8_u
          i32.store8
          local.get 3
          i32.const 5
          i32.add
          local.get 7
          i32.const 5
          i32.add
          i32.load8_u
          i32.store8
          local.get 3
          i32.const 6
          i32.add
          local.get 7
          i32.const 6
          i32.add
          i32.load8_u
          i32.store8
          local.get 3
          i32.const 7
          i32.add
          local.get 7
          i32.const 7
          i32.add
          i32.load8_u
          i32.store8
          local.get 7
          i32.const 8
          i32.add
          local.set 7
          local.get 3
          i32.const 8
          i32.add
          local.tee 3
          local.get 5
          i32.ne
          br_if 0 (;@3;)
        end
      end
      local.get 5
      local.get 2
      local.get 4
      i32.sub
      local.tee 8
      i32.const -4
      i32.and
      local.tee 6
      i32.add
      local.set 3
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          local.get 4
          i32.add
          local.tee 7
          i32.const 3
          i32.and
          br_if 0 (;@3;)
          local.get 5
          local.get 3
          i32.ge_u
          br_if 1 (;@2;)
          local.get 7
          local.set 1
          loop  ;; label = @4
            local.get 5
            local.get 1
            i32.load
            i32.store
            local.get 1
            i32.const 4
            i32.add
            local.set 1
            local.get 5
            i32.const 4
            i32.add
            local.tee 5
            local.get 3
            i32.lt_u
            br_if 0 (;@4;)
            br 2 (;@2;)
          end
        end
        local.get 5
        local.get 3
        i32.ge_u
        br_if 0 (;@2;)
        local.get 7
        i32.const 3
        i32.shl
        local.tee 2
        i32.const 24
        i32.and
        local.set 4
        local.get 7
        i32.const -4
        i32.and
        local.tee 9
        i32.const 4
        i32.add
        local.set 1
        i32.const 0
        local.get 2
        i32.sub
        i32.const 24
        i32.and
        local.set 10
        local.get 9
        i32.load
        local.set 2
        loop  ;; label = @3
          local.get 5
          local.get 2
          local.get 4
          i32.shr_u
          local.get 1
          i32.load
          local.tee 2
          local.get 10
          i32.shl
          i32.or
          i32.store
          local.get 1
          i32.const 4
          i32.add
          local.set 1
          local.get 5
          i32.const 4
          i32.add
          local.tee 5
          local.get 3
          i32.lt_u
          br_if 0 (;@3;)
        end
      end
      local.get 8
      i32.const 3
      i32.and
      local.set 2
      local.get 7
      local.get 6
      i32.add
      local.set 1
    end
    block  ;; label = @1
      local.get 3
      local.get 3
      local.get 2
      i32.add
      local.tee 5
      i32.ge_u
      br_if 0 (;@1;)
      local.get 2
      i32.const -1
      i32.add
      local.set 8
      block  ;; label = @2
        local.get 2
        i32.const 7
        i32.and
        local.tee 7
        i32.eqz
        br_if 0 (;@2;)
        loop  ;; label = @3
          local.get 3
          local.get 1
          i32.load8_u
          i32.store8
          local.get 1
          i32.const 1
          i32.add
          local.set 1
          local.get 3
          i32.const 1
          i32.add
          local.set 3
          local.get 7
          i32.const -1
          i32.add
          local.tee 7
          br_if 0 (;@3;)
        end
      end
      local.get 8
      i32.const 7
      i32.lt_u
      br_if 0 (;@1;)
      loop  ;; label = @2
        local.get 3
        local.get 1
        i32.load8_u
        i32.store8
        local.get 3
        i32.const 1
        i32.add
        local.get 1
        i32.const 1
        i32.add
        i32.load8_u
        i32.store8
        local.get 3
        i32.const 2
        i32.add
        local.get 1
        i32.const 2
        i32.add
        i32.load8_u
        i32.store8
        local.get 3
        i32.const 3
        i32.add
        local.get 1
        i32.const 3
        i32.add
        i32.load8_u
        i32.store8
        local.get 3
        i32.const 4
        i32.add
        local.get 1
        i32.const 4
        i32.add
        i32.load8_u
        i32.store8
        local.get 3
        i32.const 5
        i32.add
        local.get 1
        i32.const 5
        i32.add
        i32.load8_u
        i32.store8
        local.get 3
        i32.const 6
        i32.add
        local.get 1
        i32.const 6
        i32.add
        i32.load8_u
        i32.store8
        local.get 3
        i32.const 7
        i32.add
        local.get 1
        i32.const 7
        i32.add
        i32.load8_u
        i32.store8
        local.get 1
        i32.const 8
        i32.add
        local.set 1
        local.get 3
        i32.const 8
        i32.add
        local.tee 3
        local.get 5
        i32.ne
        br_if 0 (;@2;)
      end
    end
    local.get 0)
  (memory (;0;) 17)
  (global $__stack_pointer (mut i32) (i32.const 1048576))
  (global (;1;) i32 (i32.const 1049300))
  (global (;2;) i32 (i32.const 1049312))
  (export "memory" (memory 0))
  (export "anyblox_decode" (func $anyblox_decode))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (data $.rodata (i32.const 1048576) "/home/mat/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec.rs\00\00\00\10\00o\00\00\00(\02\00\00\11\00\00\00rle-linestatus-paged/src/lib.rs\00\80\00\10\00\1f\00\00\00J\00\00\00=\00\00\00\80\00\10\00\1f\00\00\00\f0\00\00\00\16\00\00\00called `Option::unwrap()` on a `None` value/home/mat/.cargo/registry/src/index.crates.io-6f17d22bba15001f/talc-4.4.2/src/oom_handler.rs\00\eb\00\10\00\5c\00\00\00\83\00\00\00B\00\00\00/home/mat/.cargo/registry/src/index.crates.io-6f17d22bba15001f/talc-4.4.2/src/talc.rs\00\00\00X\01\10\00U\00\00\00\dc\02\00\00\09\00\00\00assertion failed: !self.bins.is_null()\00\00X\01\10\00U\00\00\00B\03\00\00\09\00\00\00X\01\10\00U\00\00\00D\03\00\00\09\00\00\00assertion failed: old_heap.size() >= MIN_HEAP_SIZE\00\00X\01\10\00U\00\00\00C\03\00\00\09\00\00\00X\01\10\00U\00\00\00I\03\00\00Q\00\00\00X\01\10\00U\00\00\00E\03\00\00\09\00\00\00"))
