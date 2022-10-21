import http from 'k6/http';
import { check } from 'k6';

export const options = {
    stages: [
        { duration: '30s', target: 100 },
        { duration: '30s', target: 500 },
        { duration: '120s', target: 1000 },
    ],
};

export default function () {
    const url = 'http://host.docker.internal:8080/items';
    const payload = JSON.stringify({
        name: 'Hi-Potion',
        description: 'Restores a large amount of HP',
        price: 20,
        image_url: 'url',
    });

    const params = {
        headers: {
            'Content-Type': 'application/json',
        },
    };

    const res = http.post(url, payload, params);
    check(res, {
        'status is 200': (r) => r.status === 200,
    });
}
