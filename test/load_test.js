import http from 'k6/http';
import { sleep } from 'k6';

let max_vu = 5000;

export let options = {
    stages: [
        { duration: '5s', target: max_vu / 2 },
        { duration: '10s', target: max_vu },
        { duration: '1m15s', target: max_vu },
        { duration: '30s', target: 0 },
    ],
};

export default function () {
    http.get('http://localhost:6920');
    sleep(1);
}
