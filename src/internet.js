function ping_test() {
    // create an exponential backoff with to a max of 5 minutes with trying to fetch "https://www.google.com", if it fails, it will try again in 1 second, then 2, then 4, then 8, etc. if it succeeds, it will stop trying and return true
    return new Promise((resolve, reject) => {
        let backoff = 0;
        const max_backoff = 30;
        const url = "https://www.google.com";
        const ping = () => {
            fetch(url, { mode: "no-cors" })
                .then(() => resolve(true))
                .catch(() => {
                    if (backoff < max_backoff) {
                        backoff = backoff === 0 ? 1000 : backoff * 2;
                        setTimeout(ping, backoff);
                    } else {
                        resolve(false);
                    }
                });
        };
        ping();
    });
}