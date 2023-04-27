# เลือก base image ที่ต้องการ
FROM rust:1.68.2-slim-bullseye

# สร้าง directory สำหรับเก็บ source code ของ application
WORKDIR /app

# copy source code ของ application ไปยัง container
COPY main.rs .

# compile source code ของ application
RUN rustc main.rs

# กำหนดค่า environment variable สำหรับ container
ENV PORT=8080

# กำหนด command สำหรับเรียกใช้งาน application
CMD ["./main"]

