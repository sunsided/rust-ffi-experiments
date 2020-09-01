//
// Created by markus on 01.09.20.
//

#ifndef OPAQUE_POINTERS_MY_VEC_H
#define OPAQUE_POINTERS_MY_VEC_H

struct MyVec;
typedef struct MyVec MyVec;

MyVec* my_vec_new();
int my_vec_len(const MyVec*);
void my_vec_push(MyVec*, int);
int* my_vec_contents(MyVec*);
void my_vec_destroy(MyVec*);


#endif //OPAQUE_POINTERS_MY_VEC_H
