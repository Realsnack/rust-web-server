import http from 'k6/http';
import { sleep } from 'k6';

let max_vu = `${__ENV.MAX_VU}`;
let url = `${__ENV.URL}`;

export let options = {
    stages: [
        { duration: '5s', target: max_vu / 2 },
        { duration: '10s', target: max_vu },
        { duration: '1m45s', target: max_vu },
        { duration: '15s', target: 0 },
    ],
    thresholds: {
        http_req_duration: ['p(95)<10'], // 95% of requests must finish within 10ms.
        http_req_failed: ['rate<0.01'], // http errors should be less than 1%
    },
    rps: max_vu/2,
};

export default function () {
    http.get(url);
    sleep(1);
}
