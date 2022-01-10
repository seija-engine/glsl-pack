int testCoreFn() {
    return 114514;
}

const float cPI = 3.1415927;
float pow5(float n) {
    testCoreFn();
    return n * n * n * n * n;
}

struct CustomType {
    vec4 colorIntensity;
};
CustomType vs_main(float paramA, const float paramB, float cache) {
    float a = pow5(123);
    for (int i = 0; i < 10; i++) {
        float c = i;
    }
    a = cPI;
}
